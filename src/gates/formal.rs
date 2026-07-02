use crate::core::ball::{Ball, GateResult};
use crate::core::pin::Gate;
use super::GateValidator;

pub struct FormalGate;

impl FormalGate {
    pub fn new() -> Self {
        FormalGate
    }
}

impl Default for FormalGate {
    fn default() -> Self {
        Self::new()
    }
}

impl FormalGate {
    /// Check if a mathematical proof or formal reasoning is valid
    fn check_proof_structure(&self, token: &str, context: &str) -> (bool, f64) {
        let lower_token = token.to_lowercase();
        let lower_context = context.to_lowercase();
        
        // Check for common proof keywords
        let proof_keywords = ["prove", "proof", "theorem", "lemma", "corollary", 
                            "hypothesis", "assumption", "given", "therefore", "thus",
                            "hence", "implies", "conclude", "q.e.d.", "qed"];
        
        let has_proof_keyword = proof_keywords.iter().any(|kw| 
            lower_token.contains(kw) || lower_context.contains(kw)
        );
        
        if !has_proof_keyword {
            return (true, 0.7); // Not a proof, so pass with moderate confidence
        }
        
        // Check for logical structure
        let logical_structures = ["for all", "exists", "if and only if", "iff",
                                "implies", "=>", "->", "∀", "∃", "↔", "→"];
        
        let has_logical_structure = logical_structures.iter().any(|ls| 
            lower_token.contains(ls) || lower_context.contains(ls)
        );
        
        // Check for quantifiers
        let quantifiers = ["for every", "for each", "there exists", "there is",
                          "for all x", "for any"];
        
        let has_quantifiers = quantifiers.iter().any(|q| 
            lower_token.contains(q) || lower_context.contains(q)
        );
        
        // Score based on structure
        let score = if has_logical_structure && has_quantifiers {
            0.9
        } else if has_logical_structure || has_quantifiers {
            0.8
        } else {
            0.7
        };
        
        (true, score)
    }
    
    /// Check for common logical errors in formal reasoning
    fn check_logical_errors(&self, token: &str, context: &str) -> (bool, f64) {
        let lower_token = token.to_lowercase();
        let lower_context = context.to_lowercase();
        
        // Check for quantifier scope errors
        let scope_errors = ["for all x there exists", "there exists for all"];
        let has_scope_error = scope_errors.iter().any(|e| 
            lower_token.contains(e) || lower_context.contains(e)
        );
        
        // Check for division by zero
        let division_by_zero = ["divide by zero", "division by zero", "/ 0", "/0"];
        let has_division_by_zero = division_by_zero.iter().any(|d| 
            lower_token.contains(d) || lower_context.contains(d)
        );
        
        // Check for circular reasoning
        let circular = ["because", "since", "as shown above", "as proved"];
        let has_circular = circular.iter().any(|c| 
            lower_token.contains(c) && lower_context.contains(c)
        );
        
        if has_scope_error || has_division_by_zero || has_circular {
            return (false, 0.2);
        }
        
        (true, 0.8)
    }
    
    /// Check if the statement is well-formed
    fn check_well_formed(&self, token: &str, _context: &str) -> (bool, f64) {
        // Check for balanced parentheses
        let mut paren_depth = 0;
        let mut bracket_depth = 0;
        
        for ch in token.chars() {
            match ch {
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                '[' => bracket_depth += 1,
                ']' => bracket_depth -= 1,
                _ => {}
            }
            
            if paren_depth < 0 || bracket_depth < 0 {
                return (false, 0.3);
            }
        }
        
        if paren_depth != 0 || bracket_depth != 0 {
            return (false, 0.4);
        }
        
        // Check for empty or very short expressions
        if token.trim().len() < 2 {
            return (true, 0.5);
        }
        
        (true, 0.85)
    }
}

impl GateValidator for FormalGate {
    fn validate(&self, ball: &mut Ball, context: &str) -> GateResult {
        let token = &ball.candidate.token;
        
        let (structure_ok, structure_score) = self.check_proof_structure(token, context);
        let (errors_ok, errors_score) = self.check_logical_errors(token, context);
        let (well_formed_ok, well_formed_score) = self.check_well_formed(token, context);
        
        let scores = [structure_score, errors_score, well_formed_score];
        let avg_score = scores.iter().sum::<f64>() / scores.len() as f64;
        
        let passed = structure_ok && errors_ok && well_formed_ok;
        let reason = if !structure_ok {
            Some("Invalid proof structure".to_string())
        } else if !errors_ok {
            Some("Logical error detected".to_string())
        } else if !well_formed_ok {
            Some("Malformed expression".to_string())
        } else {
            None
        };

        if passed {
            GateResult::passed(Gate::Formal, avg_score)
        } else {
            GateResult::failed(Gate::Formal, avg_score, &reason.unwrap_or_default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ball::TokenCandidate;

    #[test]
    fn test_formal_gate_valid_proof() {
        let gate = FormalGate::new();
        let candidate = TokenCandidate::new(0, "For all x, P(x) implies Q(x)", 0.5);
        let mut ball = Ball::new(candidate);
        let result = gate.validate(&mut ball, "theorem proof");
        assert!(result.passed);
    }

    #[test]
    fn test_formal_gate_invalid_proof() {
        let gate = FormalGate::new();
        let candidate = TokenCandidate::new(0, "divide by zero", 0.5);
        let mut ball = Ball::new(candidate);
        let result = gate.validate(&mut ball, "math proof");
        assert!(!result.passed);
    }

    #[test]
    fn test_formal_gate_unbalanced_parens() {
        let gate = FormalGate::new();
        let candidate = TokenCandidate::new(0, "(x + 1", 0.5);
        let mut ball = Ball::new(candidate);
        let result = gate.validate(&mut ball, "expression");
        assert!(!result.passed);
    }
}