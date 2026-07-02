pub mod math;
pub mod logic;
pub mod fact;
pub mod confidence;
pub mod fallacy;
pub mod formal;

pub use math::MathGate;
pub use logic::LogicGate;
pub use fact::FactGate;
pub use confidence::ConfidenceGate;
pub use fallacy::FallacyGate;
pub use formal::FormalGate;

use crate::core::ball::{Ball, GateResult};
use crate::core::pin::{Gate, Pin};
use crate::kb::facts::KnowledgeBase;

pub trait GateValidator {
    fn validate(&self, ball: &mut Ball, context: &str) -> GateResult;
}

pub fn validate_ball(ball: &mut Ball, pins: &[Pin], context: &str, kb: &KnowledgeBase) {
    for pin in pins {
        if !pin.enabled {
            continue;
        }

        let result = match pin.gate {
            Gate::Math => math::MathGate::new().validate(ball, context),
            Gate::Logic => logic::LogicGate::new().validate(ball, context),
            Gate::Fact => fact::FactGate::new(kb).validate(ball, context),
            Gate::Confidence => confidence::ConfidenceGate::new(pin.threshold).validate(ball, context),
            Gate::Formal => formal::FormalGate::new().validate(ball, context),
        };

        ball.add_result(result);
    }
}
