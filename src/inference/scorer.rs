/// Response scorer - evaluates LLM output quality using CID gates.
/// Returns quality scores and suggested actions.
use crate::gates::fallacy::FallacyGate;
use crate::gates::math::MathGate;
use crate::gates::fact::FactGate;
use crate::gates::GateValidator;
use crate::core::ball::{Ball, TokenCandidate};
use crate::kb::facts::KnowledgeBase;
use crate::inference::bias::BiasDetector;
use crate::inference::sanity::SanityChecker;

pub struct ResponseScorer {
    fallacy_gate: FallacyGate,
    math_gate: MathGate,
    kb: KnowledgeBase,
    bias_detector: BiasDetector,
    sanity_checker: SanityChecker,
}

#[derive(Debug, Clone)]
pub struct QualityReport {
    pub overall_score: f64,
    pub math_score: f64,
    pub logic_score: f64,
    pub fact_score: f64,
    pub fallacy_score: f64,
    pub bias_score: f64,
    pub confidence: f64,
    pub action: SuggestedAction,
    pub issues: Vec<QualityIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuggestedAction {
    Accept,
    Retry,
    Escalate,
    FixAndRetry,
}

#[derive(Debug, Clone)]
pub struct QualityIssue {
    pub category: IssueCategory,
    pub description: String,
    pub severity: IssueSeverity,
    pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IssueCategory {
    Math,
    Logic,
    Fact,
    Fallacy,
    Bias,
    Sanity,
    Coherence,
}

impl std::fmt::Display for IssueCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueCategory::Math => write!(f, "Math"),
            IssueCategory::Logic => write!(f, "Logic"),
            IssueCategory::Fact => write!(f, "Fact"),
            IssueCategory::Fallacy => write!(f, "Fallacy"),
            IssueCategory::Bias => write!(f, "Bias"),
            IssueCategory::Sanity => write!(f, "Sanity"),
            IssueCategory::Coherence => write!(f, "Coherence"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueSeverity::Low => write!(f, "Low"),
            IssueSeverity::Medium => write!(f, "Medium"),
            IssueSeverity::High => write!(f, "High"),
            IssueSeverity::Critical => write!(f, "Critical"),
        }
    }
}

impl ResponseScorer {
    pub fn new() -> Self {
        ResponseScorer {
            fallacy_gate: FallacyGate::new(),
            math_gate: MathGate::new(),
            kb: KnowledgeBase::new(),
            bias_detector: BiasDetector::new(),
            sanity_checker: SanityChecker::new(),
        }
    }

    /// Score a response, returning a quality report.
    pub fn score(&self, response: &str, context: &str) -> QualityReport {
        let mut issues = Vec::new();
        let mut scores = Vec::new();

        // Check for fallacies
        let fallacies = self.fallacy_gate.detect(response);
        let fallacy_score = if fallacies.is_empty() {
            1.0
        } else {
            for f in &fallacies {
                issues.push(QualityIssue {
                    category: IssueCategory::Fallacy,
                    description: format!("{}: {}", f.name, f.description),
                    severity: if f.confidence > 0.7 { IssueSeverity::High } else { IssueSeverity::Medium },
                    confidence: f.confidence,
                });
            }
            1.0 - self.fallacy_gate.score(response)
        };
        scores.push(fallacy_score);

        // Check for biases
        let biases = self.bias_detector.detect(response);
        let bias_score = if biases.is_empty() {
            1.0
        } else {
            for b in &biases {
                issues.push(QualityIssue {
                    category: IssueCategory::Bias,
                    description: format!("{}: {}", b.name, b.description),
                    severity: if b.confidence > 0.7 { IssueSeverity::High } else { IssueSeverity::Medium },
                    confidence: b.confidence,
                });
            }
            1.0 - self.bias_detector.score(response)
        };
        scores.push(bias_score);

        // Check numeric values against physical ranges
        let sanity_score = self.check_numeric_sanity(response, &mut issues);
        scores.push(sanity_score);

        // Check math expressions using real MathGate
        let math_score = self.check_math_expressions(response, context, &mut issues);
        scores.push(math_score);

        // Check facts using real FactGate
        let fact_score = self.check_facts(response, context, &mut issues);
        scores.push(fact_score);

        // Check logic (basic patterns)
        let logic_score = self.check_logic_patterns(response, context, &mut issues);
        scores.push(logic_score);

        // Check coherence
        let coherence_score = self.check_coherence(response, &mut issues);
        scores.push(coherence_score);

        // Calculate overall score (weighted average)
        let weights = [0.15, 0.1, 0.1, 0.2, 0.2, 0.1, 0.15]; // fallacy, bias, sanity, math, fact, logic, coherence
        let overall_score = scores.iter().zip(weights.iter())
            .map(|(s, w)| s * w)
            .sum::<f64>();

        // Determine action based on score and issues
        let action = self.determine_action(overall_score, &issues);

        // Calculate confidence
        let confidence = self.calculate_confidence(overall_score, &issues);

        QualityReport {
            overall_score,
            math_score,
            logic_score,
            fact_score,
            fallacy_score,
            bias_score,
            confidence,
            action,
            issues,
        }
    }

    /// Check math expressions in text using the real MathGate.
    fn check_math_expressions(&self, text: &str, context: &str, issues: &mut Vec<QualityIssue>) -> f64 {
        let mut scores = Vec::new();

        // Extract equations (patterns like "X = Y" or "X + Y = Z")
        for word in text.split_whitespace() {
            if word.contains('=') || word.contains('+') || word.contains('-') || word.contains('*') || word.contains('/') {
                let hash: u32 = word.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
                let logit = (hash % 1000) as f64 / 1000.0;
                let candidate = TokenCandidate::new(hash, word, logit);
                let mut ball = Ball::new(candidate);
                let result = self.math_gate.validate(&mut ball, context);
                scores.push(result.score);

                if !result.passed {
                    issues.push(QualityIssue {
                        category: IssueCategory::Math,
                        description: format!("Math error in '{}': {}", word, result.reason.as_deref().unwrap_or("unknown")),
                        severity: IssueSeverity::High,
                        confidence: 1.0 - result.score,
                    });
                }
            }
        }

        // Also check full sentences for embedded math
        for sentence in text.split(['.', '!', '?', ',']) {
            let trimmed = sentence.trim();
            if trimmed.is_empty() { continue; }
            // Check if sentence contains numbers and operators
            let has_numbers = trimmed.split_whitespace().any(|w| w.parse::<f64>().is_ok());
            let has_operators = trimmed.contains('=') || trimmed.contains('+') || trimmed.contains("equals");
            if has_numbers && has_operators {
                let hash: u32 = trimmed.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
                let logit = (hash % 1000) as f64 / 1000.0;
                let candidate = TokenCandidate::new(hash, trimmed, logit);
                let mut ball = Ball::new(candidate);
                let result = self.math_gate.validate(&mut ball, context);
                scores.push(result.score);

                if !result.passed {
                    issues.push(QualityIssue {
                        category: IssueCategory::Math,
                        description: format!("Math error: {}", result.reason.as_deref().unwrap_or("invalid expression")),
                        severity: IssueSeverity::High,
                        confidence: 1.0 - result.score,
                    });
                }
            }
        }

        if scores.is_empty() { 1.0 } else { scores.iter().sum::<f64>() / scores.len() as f64 }
    }

    /// Check factual claims using the real FactGate.
    fn check_facts(&self, text: &str, context: &str, issues: &mut Vec<QualityIssue>) -> f64 {
        let mut scores = Vec::new();
        let fact_gate = FactGate::new(&self.kb);

        // Only check sentences that have BOTH a number AND a factual keyword.
        // This avoids false positives on generic text with numbers.
        for sentence in text.split(['.', '!', '?']) {
            let trimmed = sentence.trim();
            if trimmed.is_empty() { continue; }

            let lower = trimmed.to_lowercase();
            let has_number = trimmed.split_whitespace().any(|w| {
                w.replace([',', '.', '-'], "").parse::<f64>().is_ok()
            });
            let has_factual_keyword = lower.contains("is approximately")
                || lower.contains("equals")
                || lower.contains("has a mass")
                || lower.contains("speed of")
                || lower.contains("distance")
                || lower.contains("population")
                || lower.contains("density")
                || lower.contains("temperature")
                || lower.contains("is about")
                || lower.contains("is roughly");

            if has_number && has_factual_keyword {
                let hash: u32 = trimmed.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
                let logit = (hash % 1000) as f64 / 1000.0;
                let candidate = TokenCandidate::new(hash, trimmed, logit);
                let mut ball = Ball::new(candidate);
                let result = fact_gate.validate(&mut ball, context);
                scores.push(result.score);

                if !result.passed {
                    issues.push(QualityIssue {
                        category: IssueCategory::Fact,
                        description: format!("Fact check failed: {}", result.reason.as_deref().unwrap_or("unknown")),
                        severity: IssueSeverity::High,
                        confidence: 1.0 - result.score,
                    });
                }
            }
        }

        if scores.is_empty() { 1.0 } else { scores.iter().sum::<f64>() / scores.len() as f64 }
    }

    fn check_numeric_sanity(&self, text: &str, issues: &mut Vec<QualityIssue>) -> f64 {
        // Extract numbers from text and check against sanity ranges
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut sanity_scores = Vec::new();

        for i in 0..words.len() {
            if let Ok(num) = words[i].replace(',', "").parse::<f64>() {
                // Check surrounding context for unit hints
                let context_start = i.saturating_sub(3);
                let context_end = (i + 4).min(words.len());
                let _context_window = words[context_start..context_end].join(" ");

                // Try common categories
                for category in &["speed", "temperature", "height", "weight", "energy", "distance", "time"] {
                    let score = self.sanity_checker.score(num, category);
                    if score < 0.5 {
                        issues.push(QualityIssue {
                            category: IssueCategory::Sanity,
                            description: format!("{} {} may be outside typical range for {}", num, words.get(i+1).unwrap_or(&""), category),
                            severity: IssueSeverity::Medium,
                            confidence: 1.0 - score,
                        });
                    }
                    sanity_scores.push(score);
                }
            }
        }

        if sanity_scores.is_empty() {
            1.0
        } else {
            sanity_scores.iter().sum::<f64>() / sanity_scores.len() as f64
        }
    }

    fn check_logic_patterns(&self, text: &str, _context: &str, issues: &mut Vec<QualityIssue>) -> f64 {
        let lower = text.to_lowercase();
        let mut score: f64 = 1.0;

        // Check for common logical issues
        if lower.contains("always") && lower.contains("never") {
            issues.push(QualityIssue {
                category: IssueCategory::Logic,
                description: "Absolute language detected (always/never)".to_string(),
                severity: IssueSeverity::Low,
                confidence: 0.6,
            });
            score -= 0.1;
        }

        if lower.contains("everyone knows") || lower.contains("it's obvious") {
            issues.push(QualityIssue {
                category: IssueCategory::Logic,
                description: "Appeal to common knowledge".to_string(),
                severity: IssueSeverity::Medium,
                confidence: 0.7,
            });
            score -= 0.2;
        }

        if lower.contains("therefore") && !lower.contains("because") && !lower.contains("since") {
            issues.push(QualityIssue {
                category: IssueCategory::Logic,
                description: "Conclusion without stated premises".to_string(),
                severity: IssueSeverity::Low,
                confidence: 0.5,
            });
            score -= 0.1;
        }

        score.max(0.0)
    }

    fn check_coherence(&self, text: &str, issues: &mut Vec<QualityIssue>) -> f64 {
        let sentences: Vec<&str> = text.split(['.', '!', '?'])
            .filter(|s| !s.trim().is_empty())
            .collect();

        if sentences.len() < 2 {
            return 1.0; // Single sentence is always coherent
        }

        let mut score: f64 = 1.0;

        // Check for very short responses (may indicate incomplete answers)
        if text.len() < 20 && sentences.len() > 1 {
            issues.push(QualityIssue {
                category: IssueCategory::Coherence,
                description: "Response seems incomplete".to_string(),
                severity: IssueSeverity::Low,
                confidence: 0.6,
            });
            score -= 0.1;
        }

        // Check for repetition
        let words: Vec<&str> = text.split_whitespace().collect();
        let unique_words: std::collections::HashSet<&str> = words.iter().cloned().collect();
        let repetition_ratio = 1.0 - (unique_words.len() as f64 / words.len().max(1) as f64);

        if repetition_ratio > 0.5 {
            issues.push(QualityIssue {
                category: IssueCategory::Coherence,
                description: format!("High word repetition ({:.0}%)", repetition_ratio * 100.0),
                severity: IssueSeverity::Medium,
                confidence: repetition_ratio,
            });
            score -= 0.2;
        }

        score.max(0.0)
    }

    fn determine_action(&self, score: f64, issues: &[QualityIssue]) -> SuggestedAction {
        let has_critical = issues.iter().any(|i| i.severity == IssueSeverity::Critical);
        let has_high = issues.iter().any(|i| i.severity == IssueSeverity::High);

        if has_critical || score < 0.3 {
            SuggestedAction::Escalate
        } else if score < 0.5 || has_high {
            SuggestedAction::Retry
        } else if score < 0.7 {
            SuggestedAction::FixAndRetry
        } else {
            SuggestedAction::Accept
        }
    }

    fn calculate_confidence(&self, score: f64, issues: &[QualityIssue]) -> f64 {
        let base_confidence = score;
        let issue_penalty = issues.iter().map(|i| {
            match i.severity {
                IssueSeverity::Critical => 0.3,
                IssueSeverity::High => 0.2,
                IssueSeverity::Medium => 0.1,
                IssueSeverity::Low => 0.05,
            }
        }).sum::<f64>();

        (base_confidence - issue_penalty).clamp(0.0, 1.0)
    }
}

impl Default for ResponseScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_good_response() {
        let scorer = ResponseScorer::new();
        let report = scorer.score(
            "The capital of France is Paris. It is a major European city.",
            "factual question"
        );
        assert!(report.overall_score > 0.5, "Good response should score well, got {}", report.overall_score);
        assert_eq!(report.action, SuggestedAction::Accept);
    }

    #[test]
    fn test_score_biased_response() {
        let scorer = ResponseScorer::new();
        let report = scorer.score(
            "Everyone knows that this is the best approach. It's obvious that we should always do this.",
            "analysis"
        );
        assert!(!report.issues.is_empty(), "Should detect issues");
    }

    #[test]
    fn test_score_incomplete_response() {
        let scorer = ResponseScorer::new();
        let report = scorer.score("Yes.", "question");
        // Short response might get flagged
        assert!(report.confidence >= 0.0);
    }

    #[test]
    fn test_determine_action_accept() {
        let scorer = ResponseScorer::new();
        let action = scorer.determine_action(0.9, &[]);
        assert_eq!(action, SuggestedAction::Accept);
    }

    #[test]
    fn test_determine_action_escalate() {
        let scorer = ResponseScorer::new();
        let action = scorer.determine_action(0.2, &[]);
        assert_eq!(action, SuggestedAction::Escalate);
    }

    #[test]
    fn test_determine_action_with_critical_issue() {
        let scorer = ResponseScorer::new();
        let issues = vec![QualityIssue {
            category: IssueCategory::Math,
            description: "Critical math error".to_string(),
            severity: IssueSeverity::Critical,
            confidence: 0.95,
        }];
        let action = scorer.determine_action(0.8, &issues);
        assert_eq!(action, SuggestedAction::Escalate);
    }

    #[test]
    fn test_coherence_check() {
        let scorer = ResponseScorer::new();
        let mut issues = Vec::new();
        let score = scorer.check_coherence("This is a well-formed sentence with multiple clauses that flow together nicely.", &mut issues);
        assert!(score > 0.8, "Coherent text should score well");
    }

    #[test]
    fn test_logic_check() {
        let scorer = ResponseScorer::new();
        let mut issues = Vec::new();
        let score = scorer.check_logic_patterns(
            "This is always wrong and never right because the evidence shows otherwise",
            "",
            &mut issues
        );
        assert!(score < 1.0, "Absolute language should reduce score");
    }
}
