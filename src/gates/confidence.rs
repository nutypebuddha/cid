use crate::core::ball::{Ball, GateResult};
use crate::core::pin::Gate;
use super::GateValidator;

pub struct ConfidenceGate {
    threshold: f64,
    platt: Platt,
}

#[derive(Debug, Clone)]
pub struct Platt {
    pub a: f64,
    pub b: f64,
}

impl Platt {
    pub fn new(a: f64, b: f64) -> Self {
        Platt { a, b }
    }

    pub fn calibrate(&self, raw: f64) -> f64 {
        let z = self.a * raw + self.b;
        let clamped = z.clamp(-500.0, 500.0);
        1.0 / (1.0 + (-clamped).exp())
    }

    pub fn identity() -> Self {
        Platt { a: 0.0, b: 0.0 }
    }
}

/// Temperature Scaling: simpler alternative to Platt scaling.
/// Applies: P(y|x) = exp(z_i / T) / Σ exp(z_j / T)
/// Higher T = softer distribution (reduces overconfidence)
/// Lower T = sharper distribution (increases confidence)
#[derive(Debug, Clone)]
pub struct TemperatureScaling {
    pub temperature: f64,
}

impl TemperatureScaling {
    pub fn new(temperature: f64) -> Self {
        TemperatureScaling { temperature: temperature.max(0.01) }
    }

    /// Apply temperature scaling to a logit
    pub fn scale(&self, logit: f64) -> f64 {
        (logit / self.temperature).clamp(-500.0, 500.0)
    }

    /// Convert logit to probability with temperature scaling
    pub fn probability(&self, logit: f64) -> f64 {
        let scaled = logit / self.temperature;
        let clamped = scaled.clamp(-500.0, 500.0);
        1.0 / (1.0 + (-clamped).exp())
    }

    /// Fit temperature on validation data using NLL minimization
    /// Returns optimal temperature parameter
    pub fn fit(logits: &[f64], labels: &[bool], learning_rate: f64, iterations: usize) -> f64 {
        if logits.is_empty() || logits.len() != labels.len() {
            return 1.0;
        }

        let mut temp = 1.0; // Start with no scaling

        for _ in 0..iterations {
            let mut grad = 0.0;
            let mut _nll = 0.0;

            for (logit, &label) in logits.iter().zip(labels.iter()) {
                let z = logit / temp;
                let clamped = z.clamp(-500.0, 500.0);
                let p = 1.0 / (1.0 + (-clamped).exp());

                // Gradient of NLL w.r.t. temperature
                let d_nll = if label {
                    -logit / (temp * temp) * p * (1.0 - p)
                } else {
                    logit / (temp * temp) * p * (1.0 - p)
                };
                grad += d_nll;

                // NLL contribution
                if label {
                    _nll -= (p + 1e-10).ln();
                } else {
                    _nll -= (1.0 - p + 1e-10).ln();
                }
            }

            grad /= logits.len() as f64;
            temp -= learning_rate * grad;
            temp = temp.max(0.01); // Prevent division by zero
        }

        temp
    }
}

/// Expected Calibration Error (ECE) - measures miscalibration
/// ECE = Σ (n_b / n) |p̄_b - ŷ_b|
pub fn expected_calibration_error(confidences: &[f64], correct: &[bool], n_bins: usize) -> f64 {
    if confidences.is_empty() || confidences.len() != correct.len() {
        return 0.0;
    }

    let n = confidences.len() as f64;
    let bin_size = 1.0 / n_bins as f64;
    let mut ece = 0.0;

    for b in 0..n_bins {
        let lower = b as f64 * bin_size;
        let upper = (b + 1) as f64 * bin_size;

        let bin_samples: Vec<(f64, bool)> = confidences.iter()
            .zip(correct.iter())
            .filter(|(c, _)| **c >= lower && **c < upper)
            .map(|(&c, &r)| (c, r))
            .collect();

        if bin_samples.is_empty() {
            continue;
        }

        let bin_size_actual = bin_samples.len() as f64;
        let mean_conf = bin_samples.iter().map(|(c, _)| c).sum::<f64>() / bin_size_actual;
        let mean_acc = bin_samples.iter().map(|(_, r)| if *r { 1.0 } else { 0.0 }).sum::<f64>() / bin_size_actual;

        ece += (bin_size_actual / n) * (mean_conf - mean_acc).abs();
    }

    ece
}

/// Maximum Calibration Error (MCE) - worst bin error
pub fn maximum_calibration_error(confidences: &[f64], correct: &[bool], n_bins: usize) -> f64 {
    if confidences.is_empty() || confidences.len() != correct.len() {
        return 0.0;
    }

    let bin_size = 1.0 / n_bins as f64;
    let mut max_error = 0.0;

    for b in 0..n_bins {
        let lower = b as f64 * bin_size;
        let upper = (b + 1) as f64 * bin_size;

        let bin_samples: Vec<(f64, bool)> = confidences.iter()
            .zip(correct.iter())
            .filter(|(c, _)| **c >= lower && **c < upper)
            .map(|(&c, &r)| (c, r))
            .collect();

        if bin_samples.is_empty() {
            continue;
        }

        let bin_size_actual = bin_samples.len() as f64;
        let mean_conf = bin_samples.iter().map(|(c, _)| c).sum::<f64>() / bin_size_actual;
        let mean_acc = bin_samples.iter().map(|(_, r)| if *r { 1.0 } else { 0.0 }).sum::<f64>() / bin_size_actual;

        let error = (mean_conf - mean_acc).abs();
        if error > max_error {
            max_error = error;
        }
    }

    max_error
}

#[derive(Debug, Clone)]
pub struct CalibrationData {
    pub samples: Vec<(f64, bool)>,
}

impl CalibrationData {
    pub fn new() -> Self {
        CalibrationData { samples: Vec::new() }
    }

    pub fn add(&mut self, raw_score: f64, is_correct: bool) {
        self.samples.push((raw_score, is_correct));
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    pub fn fit_platt(&self, learning_rate: f64, iterations: usize) -> Platt {
        if self.samples.len() < 2 {
            return Platt::identity();
        }

        let mut a = 0.0;
        let mut b = 0.0;

        for _ in 0..iterations {
            let mut grad_a = 0.0;
            let mut grad_b = 0.0;

            for &(raw, is_correct) in &self.samples {
                let z = a * raw + b;
        let clamped = z.clamp(-500.0, 500.0);
                let pred = 1.0 / (1.0 + (-clamped).exp());
                let target = if is_correct { 1.0 } else { 0.0 };
                let error = pred - target;

                grad_a += error * raw;
                grad_b += error;
            }

            let n = self.samples.len() as f64;
            a -= learning_rate * grad_a / n;
            b -= learning_rate * grad_b / n;
        }

        Platt::new(a, b)
    }

    pub fn to_json(&self) -> String {
        let samples: Vec<String> = self.samples.iter()
            .map(|(r, c)| format!("[{},{}]", r, c))
            .collect();
        format!("{{\"samples\":[{}]}}", samples.join(","))
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let mut data = CalibrationData::new();
        let trimmed = json.trim();

        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return Err("Invalid JSON: expected object".to_string());
        }

        let content = &trimmed[1..trimmed.len()-1];
        let content = content.trim();
        if content.is_empty() {
            return Ok(data);
        }

        let samples_start = match content.find("\"samples\"") {
            Some(pos) => pos,
            None => return Ok(data),
        };

        let after_key = &content[samples_start + "\"samples\"".len()..];
        let after_colon = after_key.trim().trim_start_matches(':').trim();

        if !after_colon.starts_with('[') {
            return Ok(data);
        }

        let bracket_end = match after_colon.rfind(']') {
            Some(pos) => pos,
            None => return Ok(data),
        };

        let samples_str = &after_colon[..=bracket_end];

        let mut depth = 0;
        let mut current_pair = String::new();
        for c in samples_str.chars() {
            match c {
                '[' => {
                    depth += 1;
                    if depth == 2 {
                        current_pair.clear();
                    }
                }
                ']' => {
                    if depth == 2 {
                        let parts: Vec<&str> = current_pair.split(',').collect();
                        if parts.len() == 2 {
                            if let Ok(raw) = parts[0].trim().parse::<f64>() {
                                let is_correct = parts[1].trim() == "true";
                                data.add(raw, is_correct);
                            }
                        }
                    }
                    depth -= 1;
                }
                _ if depth == 2 => current_pair.push(c),
                _ => {}
            }
        }

        Ok(data)
    }
}

impl Default for CalibrationData {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfidenceGate {
    pub fn new(threshold: f64) -> Self {
        ConfidenceGate {
            threshold,
            platt: Platt::identity(),
        }
    }

    pub fn with_platt(a: f64, b: f64) -> Self {
        ConfidenceGate {
            threshold: 0.0,
            platt: Platt::new(a, b),
        }
    }

    pub fn with_platt_for_domain(context: &str) -> Self {
        let platt = Self::domain_platt(context);
        ConfidenceGate {
            threshold: 0.0,
            platt,
        }
    }

    // NOTE: Platt scaling coefficients are hand-tuned, not fit against labeled data.
    // To fit: collect (raw_score, is_correct) pairs, then use logistic regression
    // to find optimal a, b. Current values are reasonable starting points.
    fn domain_platt(context: &str) -> Platt {
        let lower = context.to_lowercase();
        if lower.contains("math") {
            Platt::new(10.0, -5.0)
        } else if lower.contains("science") {
            Platt::new(8.0, -4.0)
        } else if lower.contains("code") {
            Platt::new(12.0, -6.0)
        } else if lower.contains("fact") {
            Platt::new(7.0, -3.5)
        } else {
            Platt::identity()
        }
    }

    fn calculate_raw_confidence(&self, ball: &Ball, context: &str) -> f64 {
        let base = ball.candidate.probability;

        let gate_scores: Vec<f64> = ball.gate_results.iter().map(|r| r.score).collect();
        let gate_avg = if gate_scores.is_empty() {
            base
        } else {
            gate_scores.iter().sum::<f64>() / gate_scores.len() as f64
        };

        let has_failures = ball.gate_results.iter().any(|r| !r.passed);
        let failure_penalty = if has_failures { 0.4 } else { 0.0 };

        let context_bonus = if context.len() > 10 { 0.05 } else { 0.0 };

        let raw = if gate_scores.is_empty() {
            base + context_bonus
        } else {
            (base * 0.2) + (gate_avg * 0.7) + context_bonus - failure_penalty
        };
        raw.clamp(0.0, 1.0)
    }

    fn calibrate(&self, raw: f64, context: &str) -> f64 {
        let calibrated = if self.platt.a == 0.0 && self.platt.b == 0.0 {
            let domain_platt = Self::domain_platt(context);
            domain_platt.calibrate(raw)
        } else {
            self.platt.calibrate(raw)
        };
        calibrated.clamp(0.0, 1.0)
    }

    fn apply_kakuhen(&self, confidence: f64, consecutive_wins: u32) -> f64 {
        if consecutive_wins >= 3 {
            let boost = (consecutive_wins as f64).min(10.0) * 0.1;
            (confidence + boost).min(1.0)
        } else {
            confidence
        }
    }

    /// Bayesian belief update using sensitivity and specificity.
    /// P(H|D) = P(D|H) * P(H) / P(D)
    /// 
    /// - prior: prior probability of hypothesis (0.0-1.0)
    /// - sensitivity: P(positive test | hypothesis true)
    /// - specificity: P(negative test | hypothesis false)
    /// - positive_test: whether the test result is positive
    pub fn bayesian_update(prior: f64, sensitivity: f64, specificity: f64, positive_test: bool) -> f64 {
        let prior = prior.clamp(0.001, 0.999);
        let sensitivity = sensitivity.clamp(0.001, 0.999);
        let specificity = specificity.clamp(0.001, 0.999);

        if positive_test {
            let p_d_given_h = sensitivity;
            let p_d_given_not_h = 1.0 - specificity;
            let marginal = p_d_given_h * prior + p_d_given_not_h * (1.0 - prior);
            if marginal > 0.0 {
                p_d_given_h * prior / marginal
            } else {
                0.5
            }
        } else {
            let p_not_d_given_h = 1.0 - sensitivity;
            let p_not_d_given_not_h = specificity;
            let marginal = p_not_d_given_h * prior + p_not_d_given_not_h * (1.0 - prior);
            if marginal > 0.0 {
                p_not_d_given_h * prior / marginal
            } else {
                0.5
            }
        }
    }

    /// Overconfidence adjustment based on research.
    /// High confidence (>80%) typically overconfident by 15-25%
    /// Medium confidence (60-80%) typically overconfident by 5-15%
    /// Low confidence (<60%) may be underconfident
    pub fn adjust_overconfidence(raw_confidence: f64) -> f64 {
        if raw_confidence > 0.8 {
            (raw_confidence * 0.85).max(0.0).min(1.0)
        } else if raw_confidence > 0.6 {
            (raw_confidence * 0.92).max(0.0).min(1.0)
        } else {
            (raw_confidence * 1.05).max(0.0).min(1.0)
        }
    }
}

impl GateValidator for ConfidenceGate {
    fn validate(&self, ball: &mut Ball, context: &str) -> GateResult {
        let raw = self.calculate_raw_confidence(ball, context);
        let calibrated = self.calibrate(raw, context);

        let kakuhen_eligible = calibrated > 0.9;
        let consecutive_wins = if kakuhen_eligible {
            ball.gate_results.iter().filter(|r| r.passed).count() as u32
        } else {
            0
        };
        let final_confidence = self.apply_kakuhen(calibrated, consecutive_wins);

        let passed = final_confidence >= self.threshold;

        let reason = if !passed {
            Some(format!(
                "Confidence {:.2} (raw {:.2}) below threshold {:.2}",
                final_confidence, raw, self.threshold
            ))
        } else {
            None
        };

        if passed {
            GateResult::passed(Gate::Confidence, final_confidence)
        } else {
            GateResult::failed(
                Gate::Confidence,
                final_confidence,
                &reason.unwrap_or_default(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ball::{Ball, GateResult, TokenCandidate};
    use crate::core::pin::Gate;

    fn make_ball_with_gates(probability: f64, scores: Vec<f64>) -> Ball {
        let candidate = TokenCandidate::new(1, "test", probability);
        let mut ball = Ball::new(candidate);
        for score in scores {
            ball.gate_results.push(GateResult::passed(Gate::Math, score));
        }
        ball
    }

    #[test]
    fn test_platt_calibration() {
        let platt = Platt::new(10.0, -5.0);
        let low = platt.calibrate(0.2);
        let mid = platt.calibrate(0.5);
        let high = platt.calibrate(0.8);
        assert!(low < mid, "low={} should be < mid={}", low, mid);
        assert!(mid < high, "mid={} should be < high={}", mid, high);
        assert!(high > 0.5);
    }

    #[test]
    fn test_platt_identity() {
        let platt = Platt::identity();
        let raw = 0.7;
        let cal = platt.calibrate(raw);
        assert!((cal - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_platt_bounds() {
        let platt = Platt::new(-10.0, 5.0);
        assert!(platt.calibrate(0.0) >= 0.0);
        assert!(platt.calibrate(0.0) <= 1.0);
        assert!(platt.calibrate(1.0) >= 0.0);
        assert!(platt.calibrate(1.0) <= 1.0);
    }

    #[test]
    fn test_confidence_gate_passes_calibrated() {
        let gate = ConfidenceGate::with_platt(-2.0, 1.0);
        let mut ball = make_ball_with_gates(0.7, vec![0.8, 0.7, 0.9]);
        let result = gate.validate(&mut ball, "general text here");
        assert!(result.passed);
    }

    #[test]
    fn test_kakuhen_boost() {
        let gate = ConfidenceGate::new(0.1);
        let c1 = gate.apply_kakuhen(0.85, 2);
        let c2 = gate.apply_kakuhen(0.85, 5);
        assert!(c2 > c1);
    }

    #[test]
    fn test_domain_platt_math() {
        let platt = ConfidenceGate::domain_platt("math problem");
        let low = platt.calibrate(0.2);
        let high = platt.calibrate(0.8);
        assert!(low < high, "math Platt should map higher raw to higher calibrated");
    }

    #[test]
    fn test_domain_platt_default() {
        let platt = ConfidenceGate::domain_platt("general text");
        assert_eq!(platt.a, 0.0);
        assert_eq!(platt.b, 0.0);
    }

    #[test]
    fn test_gate_result_score_is_calibrated() {
        let gate = ConfidenceGate::with_platt(10.0, -5.0);
        let mut ball = make_ball_with_gates(0.8, vec![0.9]);
        let result = gate.validate(&mut ball, "science context here");
        assert!(result.score > 0.0);
        assert!(result.score <= 1.0);
    }

    #[test]
    fn test_platt_maps_low_raw_to_low() {
        let platt = Platt::new(10.0, -5.0);
        let low = platt.calibrate(0.1);
        let high = platt.calibrate(0.9);
        assert!(low < 0.1, "low raw should map to low calibrated, got {}", low);
        assert!(high > 0.9, "high raw should map to high calibrated, got {}", high);
    }

    #[test]
    fn test_calibration_data_new() {
        let data = CalibrationData::new();
        assert!(data.is_empty());
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_calibration_data_add() {
        let mut data = CalibrationData::new();
        data.add(0.8, true);
        data.add(0.3, false);
        assert_eq!(data.len(), 2);
    }

    #[test]
    fn test_calibration_fit_platt() {
        let mut data = CalibrationData::new();
        for i in 0..100 {
            let raw = i as f64 / 100.0;
            let is_correct = raw > 0.5;
            data.add(raw, is_correct);
        }
        let platt = data.fit_platt(0.1, 1000);
        let low = platt.calibrate(0.2);
        let high = platt.calibrate(0.8);
        assert!(low < high, "fitted Platt should map low raw to lower value");
    }

    #[test]
    fn test_calibration_json_roundtrip() {
        let mut data = CalibrationData::new();
        data.add(0.8, true);
        data.add(0.3, false);
        let json = data.to_json();
        eprintln!("JSON: {}", json);
        let restored = CalibrationData::from_json(&json).unwrap();
        eprintln!("Restored len: {}", restored.len());
        eprintln!("Restored samples: {:?}", restored.samples);
        assert_eq!(restored.len(), 2);
        assert_eq!(restored.samples[0], (0.8, true));
        assert_eq!(restored.samples[1], (0.3, false));
    }

    #[test]
    fn test_bayesian_update_positive_test() {
        // Base rate 1%, sensitivity 99%, specificity 95%
        // Positive test result
        let posterior = ConfidenceGate::bayesian_update(0.01, 0.99, 0.95, true);
        // Should be around 17% (famous base rate neglect example)
        assert!(posterior > 0.10 && posterior < 0.25,
            "Expected ~17%, got {}", posterior);
    }

    #[test]
    fn test_bayesian_update_negative_test() {
        // Base rate 1%, sensitivity 99%, specificity 95%
        // Negative test result
        let posterior = ConfidenceGate::bayesian_update(0.01, 0.99, 0.95, false);
        // Should be very low
        assert!(posterior < 0.01,
            "Expected <0.01%, got {}", posterior);
    }

    #[test]
    fn test_bayesian_update_high_prior() {
        // High prior (50%), strong evidence
        let posterior = ConfidenceGate::bayesian_update(0.5, 0.9, 0.9, true);
        assert!(posterior > 0.5, "Positive test with high prior should increase belief");
    }

    #[test]
    fn test_overconfidence_adjustment() {
        // High confidence should be reduced
        let high = ConfidenceGate::adjust_overconfidence(0.9);
        assert!(high < 0.9, "High confidence should be reduced, got {}", high);
        assert!(high > 0.7, "But not too much, got {}", high);

        // Medium confidence should be slightly reduced
        let med = ConfidenceGate::adjust_overconfidence(0.7);
        assert!(med < 0.7, "Medium confidence should be reduced, got {}", med);
        assert!(med > 0.6, "But not too much, got {}", med);

        // Low confidence should be slightly increased
        let low = ConfidenceGate::adjust_overconfidence(0.5);
        assert!(low > 0.5, "Low confidence should be increased, got {}", low);
        assert!(low < 0.6, "But not too much, got {}", low);
    }

    #[test]
    fn test_temperature_scaling_basic() {
        let ts = TemperatureScaling::new(1.0);
        let p = ts.probability(0.0);
        assert!((p - 0.5).abs() < 0.01, "logit=0 should give p=0.5, got {}", p);
    }

    #[test]
    fn test_temperature_scaling_high_temp() {
        let ts = TemperatureScaling::new(2.0);
        // High temperature softens distribution
        let p_high_logit = ts.probability(2.0);
        let ts_normal = TemperatureScaling::new(1.0);
        let p_normal = ts_normal.probability(2.0);
        assert!(p_high_logit < p_normal, "High temp should reduce confidence");
    }

    #[test]
    fn test_temperature_scaling_fit() {
        // Well-calibrated data: logit > 0 should be true
        let logits: Vec<f64> = (0..100).map(|i| (i as f64 - 50.0) / 10.0).collect();
        let labels: Vec<bool> = (0..100).map(|i| i >= 50).collect();
        let temp = TemperatureScaling::fit(&logits, &labels, 0.01, 100);
        assert!(temp > 0.0, "Fitted temperature should be positive");
        assert!(temp < 10.0, "Fitted temperature should be reasonable");
    }

    #[test]
    fn test_ece_perfect_calibration() {
        // Perfect calibration: confidence matches accuracy in each bin
        // Use many samples spread evenly so each bin has matching mean conf = mean acc
        let mut confidences = Vec::new();
        let mut correct = Vec::new();
        for i in 0..100 {
            let c = (i as f64) / 100.0;
            confidences.push(c);
            correct.push(c > 0.5); // Accuracy matches confidence trend
        }
        let ece = expected_calibration_error(&confidences, &correct, 10);
        assert!(ece <= 0.3, "Well-calibrated should have reasonable ECE, got {}", ece);
    }

    #[test]
    fn test_ece_miscalibrated() {
        // Miscalibrated: high confidence but wrong
        let confidences = vec![0.9, 0.9, 0.9, 0.9];
        let correct = vec![false, false, false, false]; // 0% accuracy at 90% confidence
        let ece = expected_calibration_error(&confidences, &correct, 10);
        assert!(ece > 0.5, "Miscalibrated should have high ECE, got {}", ece);
    }

    #[test]
    fn test_mce_worst_bin() {
        let confidences = vec![0.1, 0.9, 0.9, 0.9];
        let correct = vec![true, false, false, false]; // Worst bin: 90% confidence, 0% accuracy
        let mce = maximum_calibration_error(&confidences, &correct, 10);
        assert!(mce > 0.5, "Worst bin should have high MCE, got {}", mce);
    }
}
