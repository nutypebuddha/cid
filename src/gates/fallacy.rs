pub struct FallacyGate;

#[derive(Debug, Clone)]
pub struct FallacyMatch {
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub indicator: &'static str,
    pub confidence: f64,
}

impl FallacyGate {
    pub fn new() -> Self {
        FallacyGate
    }

    pub fn detect(&self, text: &str) -> Vec<FallacyMatch> {
        let lower = text.to_lowercase();
        let mut findings = Vec::new();

        for (pattern, name, category, description, conf) in FALLACY_PATTERNS {
            if lower.contains(pattern) {
                let already = findings.iter().any(|f: &FallacyMatch| f.name == name);
                if !already {
                    findings.push(FallacyMatch {
                        name,
                        category,
                        description,
                        indicator: pattern,
                        confidence: conf,
                    });
                }
            }
        }

        findings
    }

    pub fn has_fallacy(&self, text: &str) -> bool {
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

impl Default for FallacyGate {
    fn default() -> Self {
        Self::new()
    }
}

const FALLACY_PATTERNS: [(&str, &str, &str, &str, f64); 69] = [
    // Ad Hominem
    ("you are", "ad_hominem", "personal_attack", "Attacking the person rather than the argument", 0.7),
    ("you're just", "ad_hominem", "personal_attack", "Dismissing based on identity, not reasoning", 0.65),
    ("you wouldn't understand", "ad_hominem", "personal_attack", "Dismissal based on perceived capability", 0.7),
    ("what do you know", "ad_hominem", "personal_attack", "Undermining credibility without addressing content", 0.65),
    ("stupid", "ad_hominem", "personal_attack", "Name-calling instead of logical rebuttal", 0.8),
    ("idiot", "ad_hominem", "personal_attack", "Name-calling instead of addressing the argument", 0.8),
    ("moron", "ad_hominem", "personal_attack", "Insult in place of evidence", 0.8),
    ("shut up", "ad_hominem", "personal_attack", "Silencing instead of refuting", 0.7),

    // Straw Man
    ("so you're saying", "straw_man", "misrepresentation", "Misrepresenting someone's position", 0.6),
    ("you basically want", "straw_man", "misrepresentation", "Oversimplifying opponent's argument", 0.6),
    ("you're basically saying", "straw_man", "misrepresentation", "Restating someone's position incorrectly", 0.6),
    ("so what you mean is", "straw_man", "misrepresentation", "Distorting the original claim", 0.55),
    ("in other words you want", "straw_man", "misrepresentation", "Reframing to attack a weaker position", 0.6),

    // Bandwagon (Appeal to Popularity)
    ("everyone knows", "bandwagon", "appeal_to_popularity", "Widespread belief is not evidence", 0.7),
    ("everyone agrees", "bandwagon", "appeal_to_popularity", "Consensus alone does not prove truth", 0.7),
    ("everyone believes", "bandwagon", "appeal_to_popularity", "Popular belief does not prove truth", 0.7),
    ("everybody thinks", "bandwagon", "appeal_to_popularity", "Popular opinion is not proof", 0.65),
    ("nobody disagrees", "bandwagon", "appeal_to_popularity", "Claiming universal agreement without evidence", 0.65),
    ("most people", "bandwagon", "appeal_to_popularity", "Majority belief does not determine truth", 0.5),
    ("everyone is doing", "bandwagon", "appeal_to_popularity", "Widespread action does not justify it", 0.6),
    ("all the experts", "bandwagon", "appeal_to_popularity", "Appeal to unnamed consensus", 0.55),
    ("people are saying", "bandwagon", "appeal_to_popularity", "Vague appeal to unnamed groups", 0.6),
    ("many believe", "bandwagon", "appeal_to_popularity", "Number of believers does not prove truth", 0.55),
    ("widely accepted", "bandwagon", "appeal_to_popularity", "Widespread acceptance is not evidence", 0.5),

    // Appeal to Authority
    ("scientists say", "appeal_to_authority", "false_authority", "Which scientists? Methodology matters", 0.5),
    ("scientists agree", "appeal_to_authority", "false_authority", "Scientific consensus needs supporting data", 0.5),
    ("experts say", "appeal_to_authority", "false_authority", "Expert opinion needs evidence", 0.5),
    ("experts agree", "appeal_to_authority", "false_authority", "Expert consensus is not proof alone", 0.5),
    ("studies show", "appeal_to_authority", "false_authority", "Which studies? Sample size? Methodology?", 0.5),
    ("research proves", "appeal_to_authority", "false_authority", "Research supports but rarely 'proves'", 0.55),
    ("the science is settled", "appeal_to_authority", "false_authority", "Science is never fully 'settled'", 0.6),

    // Slippery Slope
    ("if we allow this then", "slippery_slope", "absurd_extrapolation", "Extreme consequences claimed without basis", 0.7),
    ("next thing you know", "slippery_slope", "absurd_extrapolation", "Extrapolating to extreme outcomes", 0.65),
    ("this will lead to", "slippery_slope", "absurd_extrapolation", "Claiming inevitable chain of consequences", 0.5),
    ("before you know it", "slippery_slope", "absurd_extrapolation", "Assuming rapid escalation", 0.6),
    ("where does it end", "slippery_slope", "absurd_extrapolation", "Implying unstoppable progression", 0.6),
    ("chaos will", "slippery_slope", "absurd_extrapolation", "Claiming extreme societal consequences", 0.7),

    // False Dilemma
    ("either you", "false_dilemma", "false_dichotomy", "Presenting only two options when more exist", 0.5),
    ("you're either with us or", "false_dilemma", "false_dichotomy", "Forced binary choice", 0.7),
    ("there are only two", "false_dilemma", "false_dichotomy", "Restricting options artificially", 0.6),
    ("if you're not", "false_dilemma", "false_dichotomy", "Forced binary framing", 0.45),

    // Circular Reasoning
    ("because it's true", "circular_reasoning", "begging_the_question", "Using conclusion as premise", 0.7),
    ("it's true because", "circular_reasoning", "begging_the_question", "Circular validation", 0.7),
    ("by definition", "circular_reasoning", "begging_the_question", "Defining away the question", 0.4),

    // Appeal to Emotion
    ("think of the children", "appeal_to_emotion", "emotional_manipulation", "Using children to bypass logic", 0.75),
    ("how would you feel", "appeal_to_emotion", "emotional_manipulation", "Appealing to feelings over facts", 0.5),
    ("if you really cared", "appeal_to_emotion", "emotional_manipulation", "Emotional guilt as argument", 0.65),
    ("that's hurtful", "appeal_to_emotion", "emotional_manipulation", "Emotional impact is not logical refutation", 0.4),

    // Sunk Cost
    ("i've already invested", "sunk_cost", "irrelevant_past", "Past investment should not determine future", 0.7),
    ("we've come too far", "sunk_cost", "irrelevant_past", "Sunk cost preventing rational decision", 0.65),
    ("we can't stop now", "sunk_cost", "irrelevant_past", "Throwing good resources after bad", 0.6),
    ("it would be a waste", "sunk_cost", "irrelevant_past", "Focusing on irrecoverable costs", 0.5),

    // Red Herring
    ("but what about", "red_herring", "irrelevant_diversion", "Deflecting to an unrelated topic", 0.5),
    ("speaking of which", "red_herring", "irrelevant_diversion", "Changing the subject", 0.45),
    ("let's not forget", "red_herring", "irrelevant_diversion", "Introducing tangential point", 0.4),

    // False Cause (Post Hoc)
    ("ever since", "false_cause", "post_hoc", "Temporal sequence does not imply causation", 0.5),
    ("it's no coincidence", "false_cause", "post_hoc", "Assuming causation from correlation", 0.6),
    ("this proves that", "false_cause", "post_hoc", "Correlation presented as proof of causation", 0.45),

    // No True Scotsman
    ("no real", "no_true_scotsman", "definitional_retreat", "Redefining criteria to exclude counterexamples", 0.6),
    ("a true", "no_true_scotsman", "definitional_retreat", "Moving goalposts with definitions", 0.5),

    // Tu Quoque (Whataboutism)
    ("but you", "tu_quoque", "whataboutism", "Deflecting by pointing to opponent's inconsistency", 0.4),
    ("what about when you", "tu_quoque", "whataboutism", "Redirecting instead of addressing", 0.5),
    ("you did the same", "tu_quoque", "whataboutism", "Deflecting with counter-accusation", 0.5),

    // Hasty Generalization
    ("in my experience", "hasty_generalization", "overgeneralization", "Personal experience is limited data", 0.35),
    ("i've seen it once", "hasty_generalization", "overgeneralization", "Single instance as proof", 0.5),
    ("all of them are", "hasty_generalization", "overgeneralization", "Universal claim from limited data", 0.45),

    // Loaded Question
    ("have you stopped", "loaded_question", "complex_question", "Question with built-in assumption", 0.7),
    ("when did you start", "loaded_question", "complex_question", "Presupposes unestablished premise", 0.5),
    ("why do you always", "loaded_question", "complex_question", "Assumes a pattern that may not exist", 0.5),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_ad_hominem() {
        let gate = FallacyGate::new();
        let findings = gate.detect("You are stupid and your argument is wrong");
        assert!(findings.iter().any(|f| f.name == "ad_hominem"));
    }

    #[test]
    fn test_detect_bandwagon() {
        let gate = FallacyGate::new();
        let findings = gate.detect("Everyone knows this is true, so it must be");
        assert!(findings.iter().any(|f| f.name == "bandwagon"));
    }

    #[test]
    fn test_detect_slippery_slope() {
        let gate = FallacyGate::new();
        let findings = gate.detect("If we allow this then next thing you know chaos will ensue");
        assert!(findings.iter().any(|f| f.name == "slippery_slope"));
    }

    #[test]
    fn test_no_fallacy() {
        let gate = FallacyGate::new();
        assert!(!gate.has_fallacy("The sky is blue because of Rayleigh scattering"));
    }

    #[test]
    fn test_score_clean() {
        let gate = FallacyGate::new();
        assert_eq!(gate.score("Hello world"), 1.0);
    }

    #[test]
    fn test_score_fallacious() {
        let gate = FallacyGate::new();
        let s = gate.score("You are an idiot and everyone knows you're wrong");
        assert!(s < 1.0);
    }

    #[test]
    fn test_multiple_fallacies() {
        let gate = FallacyGate::new();
        let findings = gate.detect("You are stupid and everyone knows it");
        assert!(findings.len() >= 2);
    }

    #[test]
    fn test_count() {
        let gate = FallacyGate::new();
        let c = gate.count("You are stupid and everyone knows it and but what about the other side");
        assert!(c >= 3);
    }

    #[test]
    fn test_fallacy_match_fields() {
        let gate = FallacyGate::new();
        let findings = gate.detect("You are stupid");
        let f = findings.iter().find(|f| f.name == "ad_hominem").unwrap();
        assert!(!f.description.is_empty());
        assert!(f.confidence > 0.0);
    }

    #[test]
    fn test_straw_man() {
        let gate = FallacyGate::new();
        let findings = gate.detect("So you're saying we should ban all cars?");
        assert!(findings.iter().any(|f| f.name == "straw_man"));
    }
}
