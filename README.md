<p align="center">
  <img src="docs/logo.svg" width="200" alt="CID — Calibrated Inference Device" />
</p>

<h1 align="center">CID — Calibrated Inference Device</h1>

<p align="center">
  <em>"hm, interesting. your LLM made an error. how... predictable."</em><br>
  <sub>— CID, probably</sub>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-unlicense-7C3AED?style=for-the-badge" alt="License" /></a>
  <a href="https://rustup.rs"><img src="https://img.shields.io/badge/rust-1.70%2B-orange?style=for-the-badge" alt="Rust" /></a>
  <a href="CHANGELOG.md"><img src="https://img.shields.io/badge/version-v0.2.0-06B6D4?style=for-the-badge" alt="Version" /></a>
  <a href="https://codeberg.org/NutypeBuddha"><img src="https://img.shields.io/badge/codeberg-Wintermore-7C3AED?style=for-the-badge" alt="Wintermore" /></a>
</p>

<p align="center">
  <strong>Per-token validation for LLMs.</strong><br>
  Math gets checked. Logic gets tested. Facts get verified. Every single time.
</p>

<p align="center">
  <a href="#-quick-start">Quick Start</a> ·
  <a href="#-validation-gates">Gates</a> ·
  <a href="#-mcp-server">MCP</a> ·
  <a href="#-why-cid">Why CID?</a> ·
  <a href="https://codeberg.org/NutypeBuddha/cid">Codeberg</a> ·
  <a href="https://github.com/nutypebuddha/cid">GitHub</a>
</p>

---

## `// system boot...`

```
 ██████╗██████╗     ██╗███████╗██████╗
██╔════╝██╔══██╗   ██╔╝██╔════╝██╔══██╗
██║     ██████╔╝  ██╔╝ █████╗  ██████╔╝
██║     ██╔══██╗ ██╔╝  ██╔══╝  ██╔══██╗
╚██████╗██║  ██║██╔╝   ███████╗██║  ██║
 ╚═════╝╚═╝  ╚═╝╚═╝    ╚══════╝╚═╝  ╚═╝
```

> **LLMs hallucinate.** They state falsehoods with confidence, make arithmetic errors, and commit logical fallacies.
> CID is the validation layer between AI output and reality.

```
LLM:  "2 + 3 = 6"
CID:  ❌ WRONG → auto-fix: "2 + 3 = 5"  [confidence: 0.99]

LLM:  "Everyone knows this is true"
CID:  ⚠️  Bandwagon fallacy detected

LLM:  "The Earth is flat"
CID:  ❌ Fact check failed  [confidence: 0.01]
```

---

## `// pipeline.exe`

**5 validation gates. 0.0045ms overhead. 1,606 facts.**

```
    ┌─────────────────────────────────────────────────────────┐
    │            CID VALIDATION PIPELINE                       │
    │                                                          │
    │  LLM Output ──→ [α Math] ──→ [β Logic] ──→ [γ Fact]   │
    │                     │            │            │          │
    │                     ▼            ▼            ▼          │
    │               Auto-fix     Fallacy      Sanity Check    │
    │                            Detect                       │
    │                             │                           │
    │                             ▼                           │
    │                     Validated Output                     │
    │                                                          │
    └─────────────────────────────────────────────────────────┘
    
    overhead: 0.0045ms  |  cost: ~$0.0000045  |  vs GPT-4o: 0.18%
```

---

## `// validation gates`

| Gate | What It Catches | Coverage |
|:-----|:----------------|:---------|
| **α Math** | Wrong equations, bad arithmetic | Arithmetic, algebra, unit conversions |
| **β Logic** | Invalid reasoning, non-sequiturs | Deductive, inductive, abductive |
| **γ Fact** | False claims, hallucinations | **1,606 facts** across 12 domains |
| **δ Confidence** | Over/under-confident statements | Platt scaling, domain calibration |
| **ε Fallacy** | 14 types of logical fallacies | 69 detection patterns |
| **ζ Bias** | 12 types of cognitive biases | 43 detection patterns |

---

## `// what can it do?`

### Validation

```bash
echo "2 + 3 = 5" | cid validate -- math
# → {"passed": true, "confidence": 0.99}

echo "The Earth is flat" | cid validate -- fact
# → {"passed": false, "confidence": 0.01}
```

### Auto-Fix

```bash
echo "2 + 3 = 6" | cid fix -- math
# → "2 + 3 = 5"

echo "hte cat sat on teh mat" | cid fix --
# → "the cat sat on the mat"
```

### Fallacy Detection

```bash
echo "Everyone knows this is true" | cid validate -- logic
# → Bandwagon fallacy detected

echo "You're wrong because you're a bot" | cid validate -- logic
# → Ad hominem fallacy detected
```

### MCP Server (22 tools)

```bash
cid mcp              # stdio mode
cid mcp-http :8080   # HTTP mode
```

Any AI agent can hook in:
```json
{"tool": "cid_validate", "text": "E=mc²", "context": "fact"}
{"tool": "cid_detect_fallacies", "text": "Everyone agrees..."}
{"tool": "cid_tanto_eval", "expression": "sqrt(144) + 3"}
```

---

## `// why cid?`

| Problem | CID Solution |
|:--------|:-------------|
| LLM math errors | α Math gate catches + auto-fixes |
| Logical fallacies | ε 14 types, 69 patterns |
| Hallucinated facts | γ 1,606 facts, 12 domains |
| Cognitive biases | ζ 12 types, 43 patterns |
| Expensive re-querying | 0.0045ms vs another LLM call |
| Platform lock-in | MCP works with Grok, Claude, GPT, Mistral |

### vs. Alternatives

| | **CID** | Guardrails | LMQL | Outlines |
|:--|:--------|:-----------|:-----|:---------|
| Language | **Rust** | Python | Python | Python |
| Binary | **630KB WASM** | ~50MB+ | ~30MB+ | ~20MB+ |
| Fallacy detection | **Yes (14)** | No | No | No |
| Fact checking | **1,606 facts** | No | No | No |
| MCP support | **22 tools** | No | No | No |
| Auto-fix | **Yes** | No | No | No |

---

## `// installation`

### From source

```bash
git clone https://codeberg.org/NutypeBuddha/cid.git
cd cid
cargo build --release
# binary: target/release/cid (~856KB)
```

### WASM (630KB — run anywhere)

```bash
rustup target add wasm32-unknown-unknown
cd cid-wasm && ./build.sh
# → cid_wasm.wasm (630KB)
```

### Termux (Android)

```bash
pkg install rust
git clone https://codeberg.org/NutypeBuddha/cid.git && cd cid
cargo build --release
```

---

## `// knowledge base`

**1,606 facts. 12 Greek-letter domains. TF-IDF search.**

| Domain | Symbol | Content |
|:-------|:------:|:--------|
| Math & Logic | α | Constants, formulas, theorems |
| Physics & Chemistry | β | Physical constants, elements |
| Astronomy | γ | Celestial bodies, distances |
| Earth & Environment | δ | Geographic, climate data |
| Biology & Medicine | ε | Biological facts, medical data |
| CS & AI | ζ | Algorithms, ML concepts |
| Engineering | η | Technical specifications |
| Economics & Finance | θ | Market data, financial ratios |
| History | ι | Historical events, dates |
| Language | κ | Linguistic facts |
| Philosophy | λ | Philosophical concepts |
| Psychology | μ | Psychological phenomena |

---

## `// architecture`

```
src/
├── main.rs              # CLI: validate, fix, mcp, tanto, proxy, score
├── lib.rs               # Public API
├── core/
│   ├── pin.rs           # Gate configuration
│   ├── ball.rs          # Token candidates + results
│   └── pocket.rs        # Selected best candidate
├── gates/
│   ├── math.rs          # Math validation + auto-fix
│   ├── logic.rs         # Reasoning chain validation
│   ├── fact.rs          # KB lookup (1,606 facts)
│   ├── confidence.rs    # Platt scaling calibration
│   ├── fallacy.rs       # 69 fallacy patterns (14 types)
│   ├── bias.rs          # 43 bias patterns (12 types)
│   └── formal.rs        # Symbolic logic verification
├── inference/
│   ├── pipeline.rs      # Validation orchestration
│   ├── proxy.rs         # Multi-provider LLM proxy
│   ├── stream.rs        # SSE streaming validation
│   └── compressor.rs    # Prompt compression (30-50%)
├── mcp/
│   ├── server.rs        # MCP JSON-RPC server
│   └── tools.rs         # 22 MCP tools
├── tanto/               # Tanto compute engine
│   ├── math.rs          # Arithmetic operations
│   ├── rational.rs      # Exact fractions
│   ├── convert.rs       # 60+ unit conversions
│   ├── formulas.rs      # 22 physics formulas
│   ├── solver.rs        # 9 solver templates
│   ├── sanity.rs        # Physical range checks
│   ├── thinking.rs      # 6 thinking frameworks
│   ├── pipeline.rs      # Expression pipelines
│   └── verify.rs        # Result verification
└── kb/
    └── facts.rs         # 1,606 facts, 12 domains, TF-IDF
```

### State Machine

```
Normal → Kakuhen → Jitan → Koatari
         (success)  (timeout)  (overflow)
```

---

## `// universal bridge`

**Any AI chatbot can use CID.** Point it at [cid-bridge](https://codeberg.org/NutypeBuddha/cid-bridge):

```bash
git clone https://codeberg.org/NutypeBuddha/cid-bridge.git
cd cid-bridge && npm install
CID_BINARY=../cid/target/release/cid npm start
```

```
POST /mcp       → Platform-aware validation (Grok, OpenAI, Anthropic, Mistral)
POST /validate  → Direct CID calls
GET  /health    → Status
```

---

## `// performance`

| Operation | Time | Cost |
|:----------|-----:|-----:|
| Math gate | ~0.001ms | ~$0.000001 |
| Logic gate | ~0.002ms | ~$0.000002 |
| Fact gate | ~0.001ms | ~$0.000001 |
| Confidence | ~0.0005ms | ~$0.0000005 |
| **Total** | **~0.0045ms** | **~$0.0000045** |

```
vs GPT-4o:  0.18% the cost
            $0.0045/Mtok  vs  $2.50/Mtok
```

---

## `// testing`

```bash
cargo test              # 181 tests (164 unit + 17 integration)
cargo bench             # criterion benchmarks
cargo clippy -- -D warnings  # 0 warnings
```

---

## `// configuration`

```bash
# Model routing
export CID_DEFAULT_MODEL="anthropic/claude-sonnet-4-6"
export CID_SMALL_MODEL="openai/gpt-4o-mini"
export CID_REASONING_MODEL="anthropic/claude-sonnet-4-6"

# Provider priority
export CID_PROVIDER_PRIORITY="anthropic,openai,google,ollama"
```

```toml
# Cargo features
[dependencies]
cid = { version = "0.2.0", features = ["proxy"] }
# - proxy: HTTP client for LLM proxy mode
# - plugins: wasmtime (v46) for WASM gate plugins
```

---

## `// links`

| | |
|:--|:--|
| **Codeberg** | [codeberg.org/NutypeBuddha/cid](https://codeberg.org/NutypeBuddha/cid) |
| **GitHub** | [github.com/nutypebuddha/cid](https://github.com/nutypebuddha/cid) |
| **Bridge** | [codeberg.org/NutypeBuddha/cid-bridge](https://codeberg.org/NutypeBuddha/cid-bridge) |
| **Author** | [Ryan Jason Phernetton](https://codeberg.org/NutypeBuddha) |

---

## `// contributing`

See [CONTRIBUTING.md](CONTRIBUTING.md).

```bash
git clone https://codeberg.org/NutypeBuddha/cid.git
cd cid
cargo build && cargo test
```

---

## `// license`

**Unlicense** — public domain. See [LICENSE](LICENSE).

---

<p align="center">
  <em>"kuru kuru~ your LLM's output has been validated."</em><br><br>
  <strong>Wintermore Housekeeping</strong> — keeping LLMs in line.
</p>
