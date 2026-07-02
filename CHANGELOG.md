# Changelog

All notable changes to CID will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-07-02

### Added
- WASM compilation target (cid-wasm crate, ~630KB)
- Tanto compute engine (12 modules: math, convert, formulas, solver, etc.)
- Universal MCP bridge ([cid-bridge](https://codeberg.org/NutypeBuddha/cid-bridge))
  - Platform adapters: Grok, OpenAI, Anthropic, Mistral, Claude
  - HTTP endpoints: `/mcp`, `/validate`, `/health`
  - nginx reverse proxy + PM2 + systemd deployment
- Woodpecker CI pipeline (build, test, WASM, clippy)
- CID validation gates: Math, Logic, Fact, Confidence, Fallacy, Bias, Formal
- Knowledge base: 776+ facts across 12 Greek-letter domains
- Multi-provider proxy: OpenAI, Anthropic, Gemini
- Plugin system: WASM-based third-party gate plugins
- Auto-fix: math errors, typos (200+), unit conversions, code consistency
- Streaming validation with SSE events
- Prompt compression (30-50% token reduction)
- Semantic cache with TF-IDF search index
- Response scoring quality evaluation
- Batch validation with parallel processing
- Dynamic model routing configuration
- Formal verification gate
- Adaptive validation depth (Quick/Standard/Full/Critical/Speculative)

### Changed
- Rust edition 2021
- Binary size: 872KB → 791KB (native), 630KB (WASM)
- Knowledge base: 600+ → 776+ facts
- Test count: 148 → 181 (164 unit + 17 integration)

### Fixed
- Unsafe code removed
- Chinese characters replaced with English
- Unused imports gated behind feature flags
- Default implementations added
- Unwrap removal in critical paths
- Nested if collapsed
- Duplicate typos removed
- Approximate constants replaced with std
- Clippy warnings resolved

## [0.0.2] - 2026-06-30

### Added
- InferenceEngine facade API
- MCP Server (7 tools over stdio JSON-RPC 2.0)
- HTTP Proxy (TCP server)
- TokenFixer auto-correction
- FallacyGate (69 patterns, 14 types)
- BiasDetector (43 patterns, 12 types)
- SanityChecker (20 physical range categories)
- Platt scaling confidence calibration
- Prompt injection detection (35 patterns)
- Knowledge base expansion (15 → 600+ facts)

### Fixed
- Math parser multi-term expressions
- Math fixer expression evaluation
- Fact gate fallback for unknown claims
- Unit conversion implementation
- Proxy HTTP client via ureq
- Stdin parsing with `---` separator

## [0.0.1] - 2026-06-29

### Added
- Initial release with core pachinko mechanics
- Four validation gates: Math, Logic, Fact, Confidence
- State machine: Normal, Kakuhen, Jitan, Koatari
- Economy system: BallEconomy, CostTracker, Budget
- Knowledge base with 15 facts
- CLI with stdin pipe mode
- Custom JSON parser (zero deps)

[Unreleased]: https://codeberg.org/NutypeBuddha/cid/compare/v0.1.0...HEAD
[0.1.0]: https://codeberg.org/NutypeBuddha/cid/releases/tag/v0.1.0
[0.0.2]: https://codeberg.org/NutypeBuddha/cid/compare/v0.0.1...v0.0.2
[0.0.1]: https://codeberg.org/NutypeBuddha/cid/releases/tag/v0.0.1
