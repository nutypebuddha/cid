pub mod core;
pub mod gates;
pub mod state;
pub mod economy;
pub mod kb;
pub mod output;
pub mod inference;
pub mod mcp;
pub mod tanto;
pub mod plugin;

pub use core::pin::{Pin, PinField, Gate};
pub use core::ball::{Ball, TokenCandidate, GateResult};
pub use core::pocket::Pocket;
pub use state::machine::{State, StateMachine, ValidationDepth};
pub use economy::tray::BallEconomy;
pub use economy::budget::Budget;
pub use economy::cost::CostTracker;
pub use economy::conversation::{ConversationTracker, DailyLimits, UsageReport};
pub use gates::{MathGate, LogicGate, FactGate, ConfidenceGate, FallacyGate, FormalGate};
pub use kb::facts::{Fact, KnowledgeBase};
pub use inference::{
    InferenceEngine, InferenceConfig, ValidationResult, TokenFix, GateScore,
    CidError, CidResult, ValidationRequest, ProxyRequest, ProxyConfig,
    Pipeline, TokenFixer,
    PromptCompressor, CompressionLevel, CompressionStats,
    SemanticCache, CacheHit, CacheStats,
    ResponseScorer, QualityReport, SuggestedAction,
};
