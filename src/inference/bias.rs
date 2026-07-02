#[derive(Debug, Clone)]
pub struct BiasMatch {
    pub name: &'static str,
    pub description: &'static str,
    pub mitigation: &'static str,
    pub indicator: &'static str,
    pub confidence: f64,
}

pub struct BiasDetector;

impl BiasDetector {
    pub fn new() -> Self {
        BiasDetector
    }

    pub fn detect(&self, text: &str) -> Vec<BiasMatch> {
        let lower = text.to_lowercase();
        let mut findings = Vec::new();

        for (pattern, name, description, mitigation, conf) in BIAS_PATTERNS {
            if lower.contains(&pattern.to_lowercase()) {
                let already = findings.iter().any(|f: &BiasMatch| f.name == name);
                if !already {
                    findings.push(BiasMatch {
                        name,
                        description,
                        mitigation,
                        indicator: pattern,
                        confidence: conf,
                    });
                }
            }
        }

        findings
    }

    pub fn has_bias(&self, text: &str) -> bool {
        !self.detect(text).is_empty()
    }

    pub fn count(&self, text: &str) -> usize {
        self.detect(text).len()
    }

    pub fn score(&self, text: &str) -> f64 {
        let findings = self.detect(text);
        if findings.is_empty() {
            return 1.0;
        }
        let total: f64 = findings.iter().map(|f| f.confidence).sum();
        (1.0 - total.min(1.0)).max(0.0)
    }
}

impl Default for BiasDetector {
    fn default() -> Self {
        Self::new()
    }
}

const BIAS_PATTERNS: [(&str, &str, &str, &str, f64); 43] = [
    // Anchoring
    ("first impression", "anchoring", "Over-relying on first impression", "Consider multiple data points before judging", 0.6),
    ("initially thought", "anchoring", "Anchored to initial thought", "Update beliefs with new evidence", 0.5),
    ("started at", "anchoring", "Anchored to starting value", "Evaluate from multiple reference points", 0.5),
    ("first number", "anchoring", "Anchored to first number seen", "Reason independently from fundamentals", 0.6),
    ("compared to the first", "anchoring", "Anchoring to initial comparison", "Use multiple comparison points", 0.5),

    // Availability
    ("I remember when", "availability", "Overweighting memorable events", "Check actual frequency data", 0.5),
    ("I saw on the news", "availability", "Media availability bias", "News overrepresents rare events", 0.6),
    ("just happened", "availability", "Recency-driven availability", "Consider long-term base rates", 0.5),
    ("vivid memory", "availability", "Vivid events feel more probable", "Statistical data beats anecdotes", 0.6),
    ("it happened to me", "availability", "Personal experience overweighted", "Personal sample size is too small", 0.5),

    // Confirmation
    ("proves my point", "confirmation", "Seeking confirming evidence", "Actively seek disconfirming evidence", 0.7),
    ("I knew it", "confirmation", "Confirmation bias — confirming existing belief", "Consider alternative explanations", 0.6),
    ("see, I told you", "confirmation", "Selective recall of correct predictions", "Track all predictions, not just hits", 0.7),
    ("this confirms", "confirmation", "Interpreting evidence as confirmation", "Ask: what would disconfirm this?", 0.6),
    ("exactly what I expected", "confirmation", "Expectation confirmation", "Test against null hypothesis", 0.5),

    // Sunk Cost
    ("already invested", "sunk_cost", "Past investment driving current decisions", "Ignore sunk costs; evaluate future only", 0.7),
    ("we've come too far", "sunk_cost", "Escalation of commitment", "Cut losses if future outlook is poor", 0.6),
    ("can't stop now", "sunk_cost", "Sunk cost preventing exit", "Past costs are irrecoverable", 0.6),
    ("wasted if we stop", "sunk_cost", "Focusing on irrecoverable costs", "Only future costs and benefits matter", 0.6),

    // Bandwagon
    ("everyone thinks", "bandwagon", "Belief based on popularity", "Popularity does not determine truth", 0.6),
    ("all my friends", "bandwagon", "Social proof bias", "Friends can share the same bias", 0.5),
    ("most people believe", "bandwagon", "Majority opinion bias", "Check independent evidence", 0.5),
    ("it's common knowledge", "bandwagon", "Assuming common knowledge is correct", "Common knowledge is often wrong", 0.6),

    // Framing
    ("90% success rate", "framing", "Positive framing bias", "Also consider: 10% failure rate", 0.5),
    ("only 1 in 10", "framing", "Negative framing effect", "Also consider: 90% survive", 0.5),
    ("glass half full", "framing", "Framing effect on perception", "Same facts, different frame", 0.4),
    ("survival rate", "framing", "Framing by survival vs mortality", "Check both survival and mortality rates", 0.5),

    // Hindsight
    ("I knew it all along", "hindsight", "Hindsight bias — believing you predicted it", "Write predictions before outcomes", 0.7),
    ("obvious in retrospect", "hindsight", "Outcome bias in hindsight", "It was not obvious before the outcome", 0.6),
    ("should have seen it coming", "hindsight", "Hindsight judgment bias", "Consider what was knowable at the time", 0.6),

    // Dunning-Kruger
    ("I'm an expert", "dunning_kruger", "Overconfidence from limited knowledge", "True expertise includes knowing limits", 0.6),
    ("it's simple", "dunning_kruger", "Oversimplifying complex topics", "Complex topics resist simple answers", 0.5),
    ("anyone can do it", "dunning_kruger", "Underestimating difficulty", "Skill causes ease, not the task being easy", 0.5),

    // Recency
    ("lately it seems", "recency", "Overweighting recent events", "Check long-term trends", 0.5),
    ("recent data shows", "recency", "Recency bias in data interpretation", "Recent data may not represent the trend", 0.4),
    ("these days", "recency", "Assuming recent pattern continues", "Long-term data is more reliable", 0.4),

    // Negativity
    ("everything is bad", "negativity", "Negativity bias — overweighting negative info", "Balance positive and negative evidence", 0.6),
    ("nothing ever works", "negativity", "Catastrophizing from negative events", "Consider base rate of success", 0.6),
    ("always fails", "negativity", "Overgeneralizing from failures", "Check actual success rate", 0.5),

    // Authority
    ("the expert said", "authority_bias", "Deferring to authority without evidence", "Experts can be wrong; check the evidence", 0.5),
    ("trust me", "authority_bias", "Claiming authority without basis", "Evaluate the argument, not the speaker", 0.5),

    // Automation Bias
    ("the algorithm decided", "automation_bias", "Over-trusting automated systems", "Verify algorithm output independently", 0.5),
    ("computer says", "automation_bias", "Blind trust in computation", "Computers can have bugs or bad data", 0.5),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_anchoring() {
        let det = BiasDetector::new();
        let findings = det.detect("My first impression was that it was correct");
        assert!(findings.iter().any(|f| f.name == "anchoring"));
    }

    #[test]
    fn test_detect_confirmation() {
        let det = BiasDetector::new();
        let findings = det.detect("This proves my point exactly");
        assert!(findings.iter().any(|f| f.name == "confirmation"));
    }

    #[test]
    fn test_detect_sunk_cost() {
        let det = BiasDetector::new();
        let findings = det.detect("We've already invested too much, can't stop now");
        assert!(findings.iter().any(|f| f.name == "sunk_cost"));
    }

    #[test]
    fn test_detect_bandwagon() {
        let det = BiasDetector::new();
        let findings = det.detect("Everyone thinks this is the best approach");
        assert!(findings.iter().any(|f| f.name == "bandwagon"));
    }

    #[test]
    fn test_no_bias() {
        let det = BiasDetector::new();
        assert!(!det.has_bias("The sky is blue due to Rayleigh scattering"));
    }

    #[test]
    fn test_score_clean() {
        let det = BiasDetector::new();
        assert_eq!(det.score("Hello world"), 1.0);
    }

    #[test]
    fn test_score_biased() {
        let det = BiasDetector::new();
        let s = det.score("I knew it all along and it proves my point");
        assert!(s < 1.0);
    }

    #[test]
    fn test_multiple_biases() {
        let det = BiasDetector::new();
        let findings = det.detect("I knew it all along and everyone thinks so");
        assert!(findings.len() >= 2);
    }

    #[test]
    fn test_bias_match_fields() {
        let det = BiasDetector::new();
        let findings = det.detect("I knew it all along");
        let f = findings.iter().find(|f| f.name == "hindsight").unwrap();
        assert!(!f.description.is_empty());
        assert!(!f.mitigation.is_empty());
        assert!(f.confidence > 0.0);
    }

    #[test]
    fn test_detect_hindsight() {
        let det = BiasDetector::new();
        let findings = det.detect("It was obvious in retrospect");
        assert!(findings.iter().any(|f| f.name == "hindsight"));
    }
}
