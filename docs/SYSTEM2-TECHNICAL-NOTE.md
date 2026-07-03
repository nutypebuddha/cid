# CID: A Non-Learned System 2 Verification Layer for AI-Generated Claims

*Technical Note v1.0 — July 2026*
*Wintermore Housekeeping / NutypeBuddha*

---

## Abstract

Current LLM verification approaches fall into two categories: (1) trained neural verifiers that inherit the same failure modes as the systems they check, or (2) handcrafted rule engines that lack the expressiveness to handle natural-language claims. We present CID (Calibrated Inference Device), a deterministic, offline-capable verification layer that occupies a distinct position in this design space: a **non-learned System 2** for AI-generated output. CID combines a custom compute engine (Tanto) with a 5-gate validation pipeline and a 1,606-fact knowledge base, achieving sub-millisecond validation at 630KB WASM footprint. We argue this architecture is uniquely suited for deployment contexts where neural verification is impractical or undesirable: air-gapped environments, regulated industries requiring auditable decision trails, and edge devices with constrained compute.

---

## 1. Introduction: The System 1 / System 2 Split in LLM Verification

Kahneman's dual-process theory (System 1: fast, intuitive, automatic; System 2: slow, deliberate, rule-following) has become an active architectural pattern in LLM research. The core insight: autoregressive language models are System 1 engines — they generate plausible tokens probabilistically, without deliberate verification of their own output.

Recent work (LLM2, NAACL 2025) explicitly builds this split into the decoding pipeline:

> "An LLM (System 1) with a process-based verifier (System 2), where the LLM proposes plausible tokens and the verifier scores them for correctness during decoding."

This framing has produced measurable improvements: accuracy gains from 50.3 to 57.8 on math reasoning benchmarks. Multiple independent papers converge on the same generator/verifier architecture, citing the same Kahneman/Evans/Stanovich lineage.

**The critical distinction we draw:** all surveyed System 2 verifiers are *trained* — a second neural network optimized via pairwise comparison loss on synthetic data. CID's verifier is *hand-specified* — deterministic rules, a real compute engine, a fact database. This is not a minor implementation detail; it is a fundamental architectural difference with concrete consequences.

---

## 2. What "Non-Learned" Means and Why It Matters

### 2.1 Inherited Failure Modes

A trained verifier processes the same input distribution as the generator. If the generator systematically misrepresents a domain (e.g., always fabricates population statistics with correct-looking but wrong magnitudes), a verifier trained on the generator's output may learn to accept these fabrications as plausible — they appear frequently in training data, and the verifier has no independent ground truth.

CID's Tanto engine evaluates claims against deterministic rules and a curated fact database. The verification path is:

```
Input claim → Parse → Evaluate (Tanto) → Compare (KB) → Pass/Fail
```

No step in this path is learned from data. The math engine evaluates `2 + 3` as `5` because that is what the arithmetic rules produce, not because it has seen this expression in training. The fact gate rejects "France population is 900 million" because the KB contains 68 million, not because it has learned to reject magnitude errors.

### 2.2 Calibration Without Training

A subtler advantage: non-learned verifiers don't require calibration. Neural classifiers output probabilities that must be calibrated (Platt scaling, temperature scaling) to be meaningful confidence scores. CID's confidence is derived from gate-level agreement: if MathGate passes and FactGate fails, confidence reflects the specific gate that failed, not a globally calibrated probability.

This matters for downstream consumers: a financial system receiving a CID validation result can interpret "confidence: 0.3, failed at FactGate" as a specific, actionable signal, rather than "confidence: 0.3" from a neural classifier where the meaning of 0.3 depends on calibration quality.

### 2.3 Determinism as a Contract

CID's output is deterministic: given the same input, same gates, same KB, it produces the same result. This is not true of neural verifiers, which involve stochastic sampling (temperature > 0) or floating-point non-determinism across hardware.

For regulated contexts (EU AI Act Article 15, SEC Rule 15c3-5), determinism is not a nice-to-have — it is the basis of auditability. An auditor can replay a CID validation and reproduce the exact decision trace. A neural verifier's decision cannot be replayed without the exact model weights, hardware, and random seed.

---

## 3. Architecture

### 3.1 The Tanto Compute Engine

Tanto is a deterministic expression evaluator written in Rust. It handles:

- **Arithmetic**: `+`, `-`, `*`, `/`, `^`, `%`, with correct operator precedence
- **Functions**: `sqrt`, `sin`, `cos`, `tan`, `exp`, `ln`, `log10`, `pow`, `hypot`
- **Constants**: `pi`, `e`, `c` (speed of light), `G` (gravitational constant), `h` (Planck's), `kB` (Boltzmann), and 20+ physical constants
- **Natural language**: "15% of 240", "sqrt(144) + 3", "avg(12 15 18)"
- **Rational arithmetic**: Exact fractions (`1/3 + 1/6 = 1/2`) without floating-point drift
- **60+ unit conversions**: mph↔km/h, F↔C↔K, lb↔kg, gal↔L, etc.
- **22 physics formulas**: circle area, kinetic energy, Ohm's law, compound interest, etc.
- **9 solver templates**: orbital mechanics, projectile motion, free fall, exponential growth, etc.

Tanto evaluates all expressions in constant time with no allocation beyond the result. Benchmark: ~0.001ms per evaluation on commodity hardware.

### 3.2 The 5-Gate Validation Pipeline

```
                    ┌─────────────────────┐
                    │    Input Claim       │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                ▼
        ┌──────────┐    ┌──────────┐    ┌──────────┐
        │ α Math   │    │ β Logic  │    │ γ Fact   │
        │ Gate     │    │ Gate     │    │ Gate     │
        └────┬─────┘    └────┬─────┘    └────┬─────┘
             │               │               │
             ▼               ▼               ▼
        ┌──────────┐    ┌──────────┐    ┌──────────┐
        │ δ Conf.  │    │ ε Fall.  │    │ ζ Bias   │
        │ Gate     │    │ Gate     │    │ Gate     │
        └────┬─────┘    └────┬─────┘    └────┬─────┘
             │               │               │
             └───────────────┼───────────────┘
                             ▼
                    ┌─────────────────────┐
                    │  Validated Output    │
                    │  + Confidence Score  │
                    │  + Decision Trace    │
                    └─────────────────────┘
```

Each gate implements the `GateValidator` trait:

```rust
pub trait GateValidator {
    fn validate(&self, ball: &mut Ball, context: &str) -> GateResult;
}
```

A `GateResult` contains: pass/fail, confidence score, gate type, and optional reason. Results are accumulated across gates; the final score is the weighted average.

### 3.3 Knowledge Base

CID ships with 1,606 curated facts across 12 Greek-letter domains:

| Domain | Symbol | Content |
|--------|--------|---------|
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

Facts are indexed by name (HashMap) and searchable via TF-IDF keyword matching. The fact gate can also extract numbers from natural language claims ("France population is 68 million") and compare against stored values with magnitude checking.

### 3.4 Deployment Modes

| Mode | Footprint | Network | Use Case |
|------|-----------|---------|----------|
| CLI | ~856KB binary | None | Local validation, scripting |
| WASM | ~630KB | None | Browser, edge, air-gapped |
| MCP Server | ~856KB + bridge | Optional | AI agent integration |
| HTTP Proxy | ~856KB + bridge | Optional | Inline LLM validation |

The WASM build is the structural differentiator: no other surveyed verification tool runs fully client-side with zero network dependency.

---

## 4. Comparison with Trained Verifiers

| Property | CID (Non-Learned) | LLM2 (Trained) | substrate-guard (SMT) |
|----------|-------------------|-----------------|----------------------|
| **Verification method** | Deterministic rules + KB | Neural network | Z3 SMT solver |
| **Training required** | No | Yes (synthetic data) | No |
| **Inherited failure modes** | None | Yes | None |
| **Calibration required** | No | Yes (Platt/temp scaling) | No |
| **Deterministic output** | Yes | No (stochastic) | Yes |
| **Offline capable** | Yes (WASM) | No (needs GPU) | Partially |
| **Binary size** | 630KB | ~GB (model weights) | ~50MB (Z3) |
| **Coverage breadth** | Narrow (math, facts, logic) | Broad (any natural language) | Broad (SMT expressiveness) |
| **Latency** | ~0.005ms | ~100ms+ | ~15ms |
| **Auditability** | Full decision trace | Black box | Full decision trace |

**Honest assessment:** CID trades breadth for speed, determinism, and deployability. A trained verifier can assess any natural-language claim; CID can only check claims that fall within its gate coverage (arithmetic, logical consistency, KB-backed numeric facts). This is a real limitation, not a marketing spin.

**The defensible claim:** for the categories CID *can* adjudicate — arithmetic correctness, unit consistency, magnitude plausibility, logical fallacy detection, cognitive bias identification — it does so with guarantees a trained verifier cannot provide: determinism, no inherited failure modes, no calibration dependency, and offline deployment.

---

## 5. Evaluation

### 5.1 Math Gate Accuracy

Tested on 2,000 arithmetic expressions (balanced mix of correct and incorrect):

| Metric | CID MathGate | Expected |
|--------|-------------|----------|
| Correct expressions accepted | 99.8% | >99% |
| Incorrect expressions rejected | 99.9% | >99% |
| Latency (p50) | 0.001ms | <1ms |
| Latency (p99) | 0.003ms | <5ms |

### 5.2 Fact Gate Magnitude Detection

Tested on 500 factual claims with deliberate magnitude errors ("France population is 900 million", "speed of light is 3,000 km/s"):

| Metric | Result |
|--------|--------|
| Magnitude errors detected | 94.2% |
| Correct claims accepted | 97.1% |
| False positive rate | 2.9% |

### 5.3 Fallacy Detection

Tested against a curated set of 200 arguments with known fallacies:

| Fallacy Type | Detection Rate |
|-------------|---------------|
| Ad hominem | 89% |
| Bandwagon | 92% |
| Straw man | 85% |
| Slippery slope | 78% |
| False dilemma | 81% |
| Circular reasoning | 76% |
| Appeal to emotion | 83% |
| **Average** | **83.4%** |

### 5.4 Comparison with substrate-guard

substrate-guard uses Z3 SMT solving for formal verification across five AI output classes, claiming 100% accuracy on 135 test cases with median latency under 15ms.

| Property | CID | substrate-guard |
|----------|-----|-----------------|
| Test cases | 2,000+ | 135 |
| Accuracy (math) | 99.8% | 100% |
| Latency (median) | 0.005ms | 15ms |
| Binary size | 630KB WASM | ~50MB |
| Offline | Yes | Partially |
| Coverage | Math, logic, facts, fallacies, bias | 5 output classes |

**Interpretation:** substrate-guard has broader coverage (SMT is more expressive than Tanto's purpose-built evaluator) and higher accuracy on its test set. CID has 3,000x lower latency and 80x smaller footprint. The right framing: CID is the *fast, embeddable* option for production systems; substrate-guard is the *thorough* option for audit-critical contexts.

---

## 6. The Composability Argument

The 2026 MCP governance landscape is converging on access control ("should this tool call happen?"). Microsoft's Agent Governance Toolkit, Google's agent control plane, and the MCP spec rewrite (shipping July 28, 2026) all address authentication and authorization — *who* can act, with *what* permissions.

None address content correctness — *is what this call produced actually correct?*

CID occupies this unclaimed layer:

```
┌─────────────────────────────────────────────────────┐
│                Agent Request                         │
│                      │                               │
│                      ▼                               │
│  ┌──────────────────────────────┐                   │
│  │  Access Control Layer         │  ← Everyone else  │
│  │  (MCP governance, OAuth 2.1) │    is building     │
│  │  "Should this call happen?"  │                   │
│  └──────────────┬───────────────┘                   │
│                 │                                    │
│                 ▼                                    │
│  ┌──────────────────────────────┐                   │
│  │  Correctness Layer (CID)     │  ← CID owns       │
│  │  "Is the content correct?"   │    this layer      │
│  └──────────────┬───────────────┘                   │
│                 │                                    │
│                 ▼                                    │
│  ┌──────────────────────────────┐                   │
│  │  Tool Execution               │                   │
│  └──────────────────────────────┘                   │
└─────────────────────────────────────────────────────┘
```

This is analogous to the distinction in distributed systems between authentication (who can act) and consensus/validation (is the value being agreed on actually correct) — two historically separate concerns. CID composes with, rather than competes against, the access-control tools everyone else is building.

---

## 7. Limitations

1. **Narrow coverage.** CID can only verify claims within its gate coverage. It cannot assess the truth of "the novel was written in 1984" (no date KB) or "this code is correct" (no program verification). Expanding coverage requires expanding the KB and gate set — a linear engineering commitment, not an architectural change.

2. **Fact KB is curated, not comprehensive.** 1,606 facts is sufficient for physics, astronomy, and mathematical constants, but insufficient for geography, history, current events, or domain-specific knowledge. The KB is extensible (dynamic fact injection is supported) but requires human curation for accuracy.

3. **No semantic understanding.** CID parses and evaluates; it does not understand. A claim like "the stock market crashed because of alien intervention" would pass CID's gates (no math errors, no logical fallacies in the sentence structure) while being semantically absurd. Semantic understanding requires a language model — which CID deliberately avoids to maintain determinism.

4. **substrate-guard outperforms on breadth.** Z3 SMT solving is more expressive than Tanto's purpose-built evaluator. For contexts where coverage breadth matters more than latency/footprint, substrate-guard is the better choice.

---

## 8. Conclusion

CID is not a general-purpose AI safety tool. It is a **lightweight, deterministic, offline-capable verification layer for the categories of AI output that can be adjudicated with certainty**: arithmetic, unit consistency, logical fallacies, cognitive biases, and KB-backed numeric claims.

The architectural position — non-learned System 2 — is genuinely distinct from both trained neural verifiers (LLM2, etc.) and general-purpose SMT solvers (substrate-guard). It trades breadth for determinism, speed, and deployability. For production systems that need verifiable math and fact checking at the edge, in air-gapped environments, or with strict audit requirements, this is the right tradeoff.

The path forward is not to compete with NeMo or Guardrails AI on breadth. It is to be the best-in-class deterministic checker for the categories Tanto can actually adjudicate, and to compose with the access-control tools everyone else is building.

---

## References

1. Kahneman, D. (2011). *Thinking, Fast and Slow*. Farrar, Straus and Giroux.
2. LLM2: Let Large Language Models Harness System 2 Reasoning. NAACL 2025. arXiv:2412.20372.
3. Type-Checked Compliance: Deterministic Guardrails for Agentic Financial Systems Using Lean 4 Theorem Proving. arXiv:2604.01483.
4. Emergent Formal Verification / substrate-guard. arXiv:2603.21149.
5. Solving Math Word Problems via Cooperative Reasoning induced Language Models. arXiv:2210.16257.
6. Securing MCP: A Control Plane for Agent Tool Execution. Microsoft for Developers, 2026.
7. EU AI Act, Article 15. Regulation (EU) 2024/1689.
8. SEC Rule 15c3-5. Regulation NMS.

---

*Wintermore Housekeeping — keeping LLMs in line.*
