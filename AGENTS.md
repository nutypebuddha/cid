# CID — Agent Instructions

## Repo layout
- **Binary**: `src/main.rs` — CLI entrypoint (gate, beam, validate, fix, mcp, tanto, proxy, compress, score)
- **Library**: `src/lib.rs` — re-exports all public modules
- **WASM**: `cid-wasm/` — separate workspace crate, wasm-bindgen bindings
- **Tests**: `tests/integration.rs` (17 integration tests), per-module `#[cfg(test)]` (164 unit tests)
- **Benches**: `benches/gate_benchmark.rs` — criterion benchmarks for all 5 gates
- **CI**: `.woodpecker.yml` — build + test + WASM + binary size
- **KB**: 1,606 facts across 12 Greek-letter domains (`facts.rs` + `facts_data.rs`)

## Build & test
```bash
cargo build --release        # size-optimized (opt-level=z, lto=fat, strip) — 856KB
cargo test --lib              # 164 unit tests
cargo test                    # + 17 integration tests (181 total)
cargo bench                   # criterion benchmarks
cargo test --features plugins # WASM plugin tests (skips on non-WASM arch)
cargo clippy -- -D warnings   # 0 warnings
```

## Binary usage
```
cid validate "<text>" <context>         → JSON: {passed, confidence, validated_text, fix_count}
cid fix "<text> --- <context>"           → human-readable output, "---" separates text from context
cid mcp                                  → stdio JSON-RPC MCP server
cid mcp-http 0.0.0.0:8080               → HTTP MCP server (22 tools)
cid tanto <eval|convert|formula|solve|think|check|estimate|pipeline|rational|verify|test>
```

## Architecture
- **5 gates**: MathGate, LogicGate, FactGate, ConfidenceGate, FormalGate — all implement `GateValidator` trait
- **3 core types**: `Pin` (gate config), `Ball` (token candidate + results), `Pocket` (selected best ball)
- **State machine**: Normal → Kakuhen → Jitan → Koatari (transitions on success/failure)
- **Economy**: BallEconomy + Budget + CostTracker — token budgeting and cost tracking
- **Inference engine**: Pipeline orchestrates validation through gates + TokenFixer for auto-correction
- **Tanto**: 12 compute modules (parser, math, rational, convert, formulas, solver, sanity, thinking, pipeline, verify, determinism, natural language)
- **KB**: 1,606 facts across 12 Greek-letter domains (Alpha through Mu), HashMap-indexed with TF-IDF search + dynamic fact injection

## Cargo features
- `proxy` → enables `ureq` HTTP client for LLM proxy mode
- `plugins` → enables `wasmtime` (v46) for WASM plugin system

## Release profile quirks
- `opt-level = "z"` (size), `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, `strip = true`
- WASM build: profiles in `cid-wasm/Cargo.toml` are **ignored** because workspace members cannot override root profile — warning is expected and harmless

## MCP server (for agent self-use)
The `cid` MCP server provides 22 tools prefixed `cid_cid_*` (e.g., `cid_cid_validate`, `cid_cid_tanto_eval`, `cid_cid_detect_fallacies`). Use them automatically for:
- Math validation before stating calculation results
- Fact/KB lookup before citing constants
- Logic validation for reasoning chains
- Fallacy/bias detection on argumentative text
- Sanity checks on numeric values
- Response quality scoring

## Bridge (external AI access)
`https://codeberg.org/NutypeBuddha/cid-bridge` — any AI chatbot hooks into CID via POST `/mcp` or `/validate`. Managed by PM2 on port 3000 (REST API) + port 8080 (MCP JSON-RPC), proxied through nginx on port 80.

## Branding
- Logo: `docs/logo.svg` (geometric witch hat, indigo/gold palette)
- Enterprise: **Wintermore Housekeeping**
- License: Unlicense (public domain)
