use crate::core::ball::Ball;
use crate::core::pocket::Pocket;
use crate::state::machine::State;

pub fn format_ball(ball: &Ball) -> String {
    let mut output = String::new();
    output.push_str(&format!("Ball #{}: \"{}\"\n", ball.candidate.id, ball.candidate.token));
    output.push_str(&format!("  Logit: {:.4}\n", ball.candidate.logit));
    output.push_str(&format!("  Probability: {:.4}\n", ball.candidate.probability));
    output.push_str(&format!("  Total Score: {:.4}\n", ball.total_score));
    output.push_str(&format!("  Validated: {}\n", ball.validated));
    
    if !ball.gate_results.is_empty() {
        output.push_str("  Gate Results:\n");
        for result in &ball.gate_results {
            let status = if result.passed { "PASS" } else { "FAIL" };
            output.push_str(&format!("    {:?}: {} (score: {:.4})\n", result.gate, status, result.score));
            if let Some(ref reason) = result.reason {
                output.push_str(&format!("      Reason: {}\n", reason));
            }
        }
    }
    
    output
}

pub fn format_pocket(pocket: &Pocket) -> String {
    let mut output = String::new();
    output.push_str("=== SELECTED TOKEN ===\n");
    output.push_str(&format!("  Token: \"{}\"\n", pocket.ball.candidate.token));
    output.push_str(&format!("  Score: {:.4}\n", pocket.ball.total_score));
    output.push_str(&format!("  Kakuhen: {}\n", pocket.kakuhen_triggered));
    output.push_str(&format!("  Confidence Boost: {:.2}x\n", pocket.confidence_boost));
    output
}

pub fn format_state(state: &State) -> String {
    match state {
        State::Normal => "State: Normal".to_string(),
        State::Kakuhen { consecutive_wins } => {
            format!("State: Kakuhen ({} consecutive wins)", consecutive_wins)
        }
        State::Jitan { fast_mode } => {
            format!("State: Jitan (fast_mode: {})", fast_mode)
        }
        State::Koatari { quick_check } => {
            format!("State: Koatari (quick_check: {})", quick_check)
        }
    }
}

pub fn format_validation_summary(candidates: &[Ball], selected: Option<&Pocket>) -> String {
    let mut output = String::new();
    output.push_str("=== VALIDATION SUMMARY ===\n");
    output.push_str(&format!("  Candidates: {}\n", candidates.len()));
    
    let validated = candidates.iter().filter(|b| b.validated).count();
    output.push_str(&format!("  Validated: {}\n", validated));
    
    let failed = candidates.len() - validated;
    output.push_str(&format!("  Failed: {}\n", failed));
    
    if let Some(pocket) = selected {
        output.push_str(&format!("  Selected: \"{}\"\n", pocket.ball.candidate.token));
        output.push_str(&format!("  Score: {:.4}\n", pocket.ball.total_score));
    } else {
        output.push_str("  Selected: None (all failed)\n");
    }
    
    output
}
