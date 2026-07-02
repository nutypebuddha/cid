# Changelog

All notable changes to CID will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-07-02

### Added
- Knowledge base expansion: 770+ new facts (total: 1,606) across all 12 domains
  - Alpha: Constants (Feigenbaum, Golomb-Dickman, Viswanath), number theory (30 Mersenne primes, 30 Fibonacci, 20 Catalan), geometry (Platonic/Archimedean solids), calculus (gamma, zeta, harmonic), trigonometry
  - Beta: Fundamental constants (Planck units, fine-structure), 30+ material densities, 20+ boiling/melting points, 20+ bond energies, ionization energies (20 elements), particle masses (pion, kaon, Higgs, W/Z, top)
  - Gamma: Solar system bodies (mass, radius, moons for all 8 planets), black holes (Sgr A*, M87*), exoplanet count, Hubble/JWST launch years
  - Delta: Geographic records (mountains, rivers, lakes, oceans), climate data (CO₂, sea level, ocean pH), tectonic plates
  - Epsilon: Human body (neurons, genes, cells), species counts (10 phyla), global health statistics
  - Zeta: Programming language history (35 languages with years), internet history (ARPANET→ChatGPT), ML model parameters and benchmarks
  - Eta: Megastructures (Burj Khalifa, Three Gorges), spacecraft (ISS, JWST, Starship), technology firsts (transistor, IC, microprocessor, MRI, heart transplant)
  - Theta: GDP data (10 countries), company revenues (Meta, Tesla, Walmart, Nvidia), global debt, AI market projection
  - Iota: Human evolution timeline, civilization history (Sumer→Cold War), historical populations, empire sizes
  - Kappa: Language speakers (20 languages), writing systems, language families
  - Lambda: Philosopher birth years (Thales→Rawls), schools of thought (Stoicism→Positive Psychology)
  - Mu: Brain stats, global disorder prevalence, classic psychology experiments, therapy origins
- Dynamic fact injection via `cid_cid_kb_add` MCP tool
- Context-aware KB retrieval via `cid_cid_kb_context` MCP tool
- `Default` trait implementation for `TantoEnv`

### Changed
- **Version bumped to v0.2.0**
- Knowledge base: 776+ → 1,606 facts (+107%)
- Binary size: 791KB → 856KB (due to KB expansion)
- All agent descriptions in opencode.json updated to v0.2.0
- Configuration references fixed (broken /tmp/cid-v0.7.0 and /root/tanto paths)

### Fixed
- All 44 clippy errors resolved (approx_constant, excessive_precision, manual_clamp, empty_line_after_doc_comment, sort_by→sort_by_key, vec_init_then_push, loop_indexing, identical_blocks)
- All 31 clippy warnings resolved (auto-fix applied via `cargo clippy --fix`)
- Broken opencode.json references to non-existent paths
- `TantoEnv` now implements `Default` trait
- `get_constant` in `tanto/mod.rs` now uses `std::f64::consts` where possible
- `gas_temp` computation in `tanto/math.rs` uses `get_constant("R_air")` instead of hardcoded value

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

[Unreleased]: https://codeberg.org/NutypeBuddha/cid/compare/v0.2.0...HEAD
[0.2.0]: https://codeberg.org/NutypeBuddha/cid/releases/tag/v0.2.0
[0.1.0]: https://codeberg.org/NutypeBuddha/cid/releases/tag/v0.1.0
[0.0.2]: https://codeberg.org/NutypeBuddha/cid/compare/v0.0.1...v0.0.2
[0.0.1]: https://codeberg.org/NutypeBuddha/cid/releases/tag/v0.0.1
