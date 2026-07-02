# Changelog

All notable changes to CID will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-07-02

### Added
- WASM compilation target (cid-wasm crate)
- Tanto compute engine integration (12 modules)
- Adaptive validation depth system
- Gate result caching with TTL and LRU eviction
- Batch validation with parallel processing
- Context embedding cache for semantic search
- TF-IDF search index for knowledge base
- Formal verification gate
- Speculative validation (fast path)
- Dynamic model routing configuration

### Changed
- Updated to Rust edition 2021
- Improved binary size optimization (615KB WASM)
- Enhanced test coverage (371 tests)

## [0.8.0] - 2026-07-02

### Added
- **WASM Support**: Compiled to WebAssembly (615KB optimized)
- **Tanto Integration**: 12 compute modules (math, formulas, solver, etc.)
- **Adaptive Validation**: Quick/Standard/Full/Critical/Speculative modes
- **Gate Caching**: TTL-based caching with LRU eviction
- **Batch Validation**: Parallel processing with rayon
- **Context Cache**: Embedding cache for semantic search
- **Formal Gate**: Symbolic logic verification
- **Speculative Validation**: Fast path with fallback

### Changed
- Binary size: 872KB → 615KB (WASM)
- Test count: 148 → 371
- Knowledge base: 600+ → 776+ facts

## [0.4.0] - 2026-06-30

### Added
- **Energy Efficiency Mode**: Tools to maximize LLM efficiency
- **Prompt Compressor**: 30-50% token reduction
- **Semantic Cache**: Meaning-based response caching
- **Response Scorer**: Quality evaluation without re-querying
- **Conversation Tracker**: Daily token usage tracking
- **MCP HTTP Transport**: HTTP server for Claude web connection
- **8th MCP Tool**: `cid_sample` for LLM completion
- **Bayesian Updating**: Confidence adjustment
- **Overconfidence Adjustment**: Research-backed corrections
- **Argument Strength Scoring**: Weighted evidence/premises
- **Hidden Assumption Detection**: 9 pattern-based assumptions
- **Reasoning Quality Scoring**: Positive/negative pattern scoring
- **Temperature Scaling**: Alternative to Platt scaling
- **ECE/MCE Metrics**: Calibration error measurement
- **Prompt Injection Detection**: 35 patterns
- **Termux Guide**: Android deployment documentation

### Changed
- Version: 0.3.0 → 0.4.0
- Test count: 104 → 148 (137 unit + 11 integration)
- Binary size: 814KB → 872KB
- FactGate: Uses `&KnowledgeBase` reference (no cloning)
- KB Lookup: O(1) via HashMap index
- Platt Calibration: Data-driven with gradient descent

### Fixed
- Unsafe code removed
- Chinese characters replaced with English
- Unused imports gated behind feature flags
- Default implementations added
- Unwrap removal
- Nested if collapsed
- Duplicate typos removed
- Approximate constants replaced with std
- Clippy warnings fixed

## [0.2.0] - 2026-06-30

### Added
- **InferenceEngine**: Facade API with `validate()`, `fix()`, `validate_token()`, `validate_beam()`
- **MCP Server**: 7 tools over stdio JSON-RPC 2.0
- **HTTP Proxy**: TCP server with `/v1/validate`, `/v1/proxy`, `/v1/health`, `/v1/stats`
- **TokenFixer**: Auto-correction for math, typos (200+), units, code consistency
- **FallacyGate**: 69 patterns across 14 fallacy types
- **BiasDetector**: 43 patterns across 12 cognitive bias types
- **SanityChecker**: 20 physical range categories
- **Platt Scaling**: Domain-specific confidence calibration
- **Knowledge Base**: Expanded from 15 to 600+ facts
- **eval.sh**: Comprehensive evaluation harness (43 test cases)

### Fixed
- Math parser: Multi-term expressions
- Math fixer: Expression evaluator for multi-term corrections
- Fact gate: Fallback returns `(false, 0.3)` for unknown claims
- Unit conversion: Implemented actual conversions
- Proxy: Added real HTTP client via `ureq`
- Stdin parsing: Multi-word text with `---` separator
- CLI logit: Token ID and logit derived from hash

### Changed
- Binary size: 781KB → 814KB
- Test count: 69 → 80 (69 unit + 11 integration)

## [0.1.0] - 2026-06-29

### Added
- Initial release with core pachinko mechanics
- Four validation gates: Math, Logic, Fact, Confidence
- State machine: Normal, Kakuhen, Jitan, Koatari
- Economy system: BallEconomy, CostTracker, Budget
- Knowledge base with 15 facts
- CLI with stdin pipe mode
- Custom JSON parser (zero deps)
- HTTP proxy (stub)

[Unreleased]: https://codeberg.org/NutypeBuddha/cid/compare/v0.8.0...HEAD
[0.8.0]: https://codeberg.org/NutypeBuddha/cid/compare/v0.4.0...v0.8.0
[0.4.0]: https://codeberg.org/NutypeBuddha/cid/compare/v0.2.0...v0.4.0
[0.2.0]: https://codeberg.org/NutypeBuddha/cid/compare/v0.1.0...v0.2.0
[0.1.0]: https://codeberg.org/NutypeBuddha/cid/releases/tag/v0.1.0
