use crate::core::ball::{Ball, GateResult};
use crate::core::pin::Gate;
use crate::kb::facts::KnowledgeBase;
use super::GateValidator;

pub struct FactGate<'a> {
    kb: &'a KnowledgeBase,
}

impl<'a> FactGate<'a> {
    pub fn new(kb: &'a KnowledgeBase) -> Self {
        FactGate { kb }
    }

    /// Extract numbers from natural language text.
    /// Handles: "68 million", "3.2 billion", "900 million", "299,792,458"
    fn extract_numbers(text: &str) -> Vec<f64> {
        let mut numbers = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        
        for (i, word) in words.iter().enumerate() {
            let clean = word.trim_matches(|c: char| !c.is_ascii_digit() && c != '.' && c != '-');
            if let Ok(num) = clean.parse::<f64>() {
                // Check next word for multiplier
                let multiplier = if i + 1 < words.len() {
                    match words[i + 1].to_lowercase().as_str() {
                        "million" | "m" => 1_000_000.0,
                        "billion" | "b" => 1_000_000_000.0,
                        "trillion" | "t" => 1_000_000_000_000.0,
                        "thousand" | "k" => 1_000.0,
                        "hundred" | "h" => 100.0,
                        _ => 1.0,
                    }
                } else {
                    1.0
                };
                numbers.push(num * multiplier);
            }
        }
        numbers
    }

    /// Extract unit hints from text (e.g., "population of France is 68 million")
    fn extract_unit_hint(text: &str) -> Option<String> {
        let lower = text.to_lowercase();
        let unit_patterns = [
            ("population", "population"),
            ("speed of", "speed"),
            ("mass of", "mass"),
            ("distance", "distance"),
            ("temperature", "temperature"),
            ("density", "density"),
            ("radius", "radius"),
            ("diameter", "diameter"),
            ("height", "height"),
            ("weight", "weight"),
            ("energy", "energy"),
            ("force", "force"),
            ("pressure", "pressure"),
            ("volume", "volume"),
            ("area", "area"),
            ("luminosity", "luminosity"),
        ];
        
        for (pattern, unit) in &unit_patterns {
            if lower.contains(pattern) {
                return Some(unit.to_string());
            }
        }
        None
    }

    /// Find KB facts that match the context by unit hint or keyword.
    fn find_matching_facts(&self, context: &str, unit_hint: &Option<String>) -> Vec<&crate::kb::facts::Fact> {
        let lower_context = context.to_lowercase();
        let mut matches = Vec::new();
        
        for fact in &self.kb.facts {
            // Match by unit hint
            if let Some(ref hint) = unit_hint {
                if fact.unit.to_lowercase().contains(hint) || fact.name.to_lowercase().contains(hint) {
                    matches.push(fact);
                    continue;
                }
            }
            // Match by context keywords
            if lower_context.contains(&fact.name.to_lowercase()) {
                matches.push(fact);
            }
            // Match by domain keywords in fact name
            let fact_words: Vec<&str> = fact.name.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()).collect();
            for fw in &fact_words {
                if lower_context.contains(&fw.to_lowercase()) {
                    matches.push(fact);
                    break;
                }
            }
        }
        matches
    }

    fn check_fact_exists(&self, token: &str, context: &str) -> (bool, f64) {
        let lower_token = token.to_ascii_lowercase();
        let lower_context = context.to_ascii_lowercase();

        // Direct lookup
        if let Some(fact) = self.kb.lookup(&lower_token) {
            let source_score = if fact.source.is_empty() { 0.9 } else { 0.95 };
            return (true, source_score);
        }

        // Check if any fact name appears in token or context
        for fact in &self.kb.facts {
            if lower_context.contains(&fact.name) || lower_token.contains(&fact.name) {
                return (true, 0.95);
            }
        }

        // Pure numbers pass (will be checked for consistency later)
        if token.bytes().all(|c| c.is_ascii_digit() || c == b'.' || c == b'-' || c == b'+' || c == b'e' || b'E' == c) {
            return (true, 0.8);
        }

        // Keyword search
        if !self.kb.search(&lower_token).is_empty() || !self.kb.search(&lower_context).is_empty() {
            return (true, 0.85);
        }

        let is_factual = token.parse::<f64>().is_ok()
            || token.contains("speed")
            || token.contains("density")
            || token.contains("mass")
            || token.contains("distance")
            || token.contains("temperature")
            || token.contains("pressure")
            || token.contains("energy")
            || token.contains("force")
            || token.contains("volume")
            || token.contains("area");
        if is_factual {
            (false, 0.3)
        } else {
            (true, 0.6)
        }
    }

    fn check_fact_consistent(&self, token: &str, context: &str) -> (bool, f64) {
        let lower_token = token.to_lowercase();
        let lower_context = context.to_lowercase();

        // Only do NL number extraction when we have a specific unit hint.
        // Generic contexts like "number", "general", "text" should not trigger KB matching.
        let unit_hint = Self::extract_unit_hint(context);
        if unit_hint.is_some() {
            let token_numbers = Self::extract_numbers(token);
            if let Some(&claimed_value) = token_numbers.first() {
                let matching_facts = self.find_matching_facts(context, &unit_hint);
                
                for fact in &matching_facts {
                    if fact.value != 0.0 {
                        let ratio = claimed_value / fact.value;
                        // Within 10% = consistent
                        if (0.9..=1.1).contains(&ratio) {
                            return (true, 0.95);
                        }
                        // Within factor of 10 = maybe wrong order of magnitude
                        if (0.1..=10.0).contains(&ratio) {
                            return (false, 0.3);
                        }
                        // Way off = inconsistent
                        return (false, 0.1);
                    }
                }
            }
        }

        // Fallback: check direct number-to-number consistency (both token and context are numbers)
        let token_numbers = Self::extract_numbers(token);
        let context_numbers = Self::extract_numbers(context);
        if let (Some(&token_val), Some(&context_val)) = (token_numbers.first(), context_numbers.first()) {
            if context_val != 0.0 {
                let ratio = token_val / context_val;
                if (0.9..=1.1).contains(&ratio) {
                    return (true, 0.9);
                } else {
                    return (false, 0.3);
                }
            }
        }

        // Check against direct KB lookup
        if let (Ok(token_val), Some(fact)) = (lower_token.parse::<f64>(), self.kb.lookup(&lower_context)) {
            if fact.value != 0.0 {
                let ratio = token_val / fact.value;
                if (0.9..=1.1).contains(&ratio) {
                    return (true, 0.95);
                }
            }
        }

        (true, 0.75)
    }

    fn check_source_reliable(&self, _token: &str, context: &str) -> (bool, f64) {
        let reliable_sources = ["wikipedia", "nasa", "nih", "nist", "ieee", "acm", "nature", "science"];
        let lower_context = context.to_lowercase();
        
        for source in &reliable_sources {
            if lower_context.contains(source) {
                return (true, 0.95);
            }
        }
        
        if context.is_empty() {
            return (true, 0.7);
        }
        
        (true, 0.8)
    }

    fn check_no_contradiction(&self, token: &str, context: &str) -> (bool, f64) {
        let lower_token = token.to_lowercase();
        let lower_context = context.to_lowercase();
        
        let fact_contradictions = [
            ("sun is cold", "sun is hot"),
            ("earth is flat", "earth is round"),
            ("water is dry", "water is wet"),
            ("light is slow", "light is fast"),
        ];
        
        for (false_fact, true_fact) in &fact_contradictions {
            if lower_context.contains(false_fact) && lower_token.contains(true_fact) {
                return (false, 0.1);
            }
            if lower_context.contains(true_fact) && lower_token.contains(false_fact) {
                return (false, 0.1);
            }
        }
        
        (true, 0.85)
    }
}

impl<'a> GateValidator for FactGate<'a> {
    fn validate(&self, ball: &mut Ball, context: &str) -> GateResult {
        let token = &ball.candidate.token;
        
        let (exists_ok, exists_score) = self.check_fact_exists(token, context);
        let (consistent_ok, consistent_score) = self.check_fact_consistent(token, context);
        let (reliable_ok, reliable_score) = self.check_source_reliable(token, context);
        let (no_contradiction, contradiction_score) = self.check_no_contradiction(token, context);
        
        let scores = [exists_score, consistent_score, reliable_score, contradiction_score];
        let avg_score = scores.iter().sum::<f64>() / scores.len() as f64;
        
        let passed = exists_ok && consistent_ok && reliable_ok && no_contradiction;
        let reason = if !exists_ok {
            Some("Fact not found in knowledge base".to_string())
        } else if !consistent_ok {
            Some("Fact inconsistent with context".to_string())
        } else if !reliable_ok {
            Some("Source reliability uncertain".to_string())
        } else if !no_contradiction {
            Some("Fact contradicts known information".to_string())
        } else {
            None
        };

        if passed {
            GateResult::passed(Gate::Fact, avg_score)
        } else {
            GateResult::failed(Gate::Fact, avg_score, &reason.unwrap_or_default())
        }
    }
}
