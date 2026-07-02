# CID v0.5.0 — Long-Term Planning & Dev Time Savings

**Optimized Plan Document**
**Date:** 2026-06-30
**Status:** Research Complete, Ready for Implementation

---

## Executive Summary

CID currently validates LLM output (catches errors). This plan extends CID to also **plan**, **decompose**, **remember**, and **automate** — turning it from a quality gate into a full development intelligence layer.

**Target:** 60-80% LLM quality boost, 11+ hours/week dev time savings, persistent long-term planning.

**Key Research Findings:**

| Technique | Source | Result |
|-----------|--------|--------|
| FLARE Lookahead | Wang et al., 2026 | LLaMA-8B beats GPT-4o |
| Task Decomposition | CMU, 2025 | +24% on SWE-Bench |
| Tree of Thoughts | Yao et al., 2023 | 4% → 74% on Game of 24 |
| Graph of Thoughts | Besta et al., 2024 | +62% over ToT, -31% cost |
| Multi-Agent Dev | Ivern AI, 2026 | 11.4 hrs/week saved |
| ReAct Pattern | Yao et al., 2022 | 6% hallucination vs 56% CoT |
| Self-Consistency | Wang et al., 2022 | +5-15pp accuracy |
| Chain-of-Verification | Meta AI, 2024 | +40% on technical writing |

---

## 1. Architecture Overview

```
USER GOAL
    │
    ▼
┌─────────────────────────────────────────────────┐
│              CID PLANNING LAYER                 │
│                                                 │
│  ┌───────────┐ ┌────────────┐ ┌──────────────┐ │
│  │   GOAL    │ │  DECOMPOSE │ │  LOOKAHEAD   │ │
│  │  TRACKER  │ │   ENGINE   │ │   (FLARE)    │ │
│  └─────┬─────┘ └─────┬──────┘ └──────┬───────┘ │
│        │             │               │          │
│  ┌─────▼─────────────▼───────────────▼──────┐  │
│  │           PLAN GENERATOR                 │  │
│  │     (Structured JSON + Validation)       │  │
│  └──────────────────┬───────────────────────┘  │
└─────────────────────┼───────────────────────────┘
                      │
         ┌────────────┼────────────┐
         ▼            ▼            ▼
    ┌─────────┐  ┌─────────┐  ┌─────────┐
    │ STEP 1  │  │ STEP 2  │  │ STEP N  │
    │  (LLM)  │  │  (LLM)  │  │  (LLM)  │
    └────┬────┘  └────┬────┘  └────┬────┘
         │            │            │
         ▼            ▼            ▼
    ┌─────────┐  ┌─────────┐  ┌─────────┐
    │ VERIFY  │  │ VERIFY  │  │ VERIFY  │
    │  CID    │  │  CID    │  │  CID    │
    │  Gates  │  │  Gates  │  │  Gates  │
    └────┬────┘  └────┬────┘  └────┬────┘
         │            │            │
         └────────────┼────────────┘
                      │
                ┌─────▼─────┐
                │  MEMORY   │ ← Facts, progress, lessons
                │  PERSIST  │
                └─────┬─────┘
                      │
                 FINAL PLAN
```

---

## 2. New MCP Tools (8 Tools)

| Tool | Purpose | Input | Output |
|------|---------|-------|--------|
| `cid_plan` | Generate structured plan | goal, context, constraints | JSON plan |
| `cid_decompose` | Break task into steps | task description | step list |
| `cid_lookahead` | Evaluate plan before execution | plan | risk assessment, score |
| `cid_track` | Track progress, update status | plan_id, step_id, status | updated plan |
| `cid_verify_plan` | Verify plan feasibility | plan | issues, suggestions |
| `cid_select_strategy` | Choose best approach | options[] | best option + reasoning |
| `cid_memory_store` | Store facts/lessons | key, value, type | confirmation |
| `cid_memory_recall` | Retrieve past knowledge | query | relevant memories |

---

## 3. Implementation Phases

### Phase 1: Goal Tracker & Plan Generator

**File:** `src/agent/planner.rs` (~400 lines)

**Goal Tracker:**
- Parse user goal into structured format
- Track: goal, sub-goals, milestones, status, deadline
- Store in memory for persistence

**Plan Generator:**
- Input: goal, context, constraints
- Output: structured JSON plan

**Plan Schema:**
```json
{
  "plan": {
    "id": "plan_001",
    "goal": "Build REST API for user management",
    "created_at": "2026-06-30T10:00:00Z",
    "status": "in_progress",
    "steps": [
      {
        "id": 1,
        "description": "Design database schema",
        "dependencies": [],
        "estimated_time": "30min",
        "status": "completed",
        "success_criteria": "Schema supports CRUD operations",
        "risks": []
      },
      {
        "id": 2,
        "description": "Implement auth middleware",
        "dependencies": [1],
        "estimated_time": "45min",
        "status": "in_progress",
        "success_criteria": "JWT validation works",
        "risks": ["Token expiry edge cases"]
      }
    ],
    "milestones": ["Schema done", "API endpoints live", "Tests passing"],
    "total_estimated_time": "3h",
    "risk_level": "low"
  }
}
```

**Key Design Decisions:**
- JSON format for machine parsing
- Dependencies enable parallel execution
- Success criteria enable automated verification
- Risk tracking enables proactive mitigation

---

### Phase 2: Task Decomposition Engine

**File:** `src/agent/decompose.rs` (~350 lines)

**Decomposition Strategy (ACONIC-inspired):**
1. Analyze task constraints
2. Identify weak coupling points
3. Split into independent subtasks
4. Order by dependency
5. Assign success criteria to each

**Decomposition Levels:**
- **Level 1:** High-level milestones (3-7 items)
- **Level 2:** Detailed steps per milestone (5-15 items)
- **Level 3:** Individual actions per step (1-5 items)

**Example:**
```
Goal: "Build user management API"
├── Milestone 1: Database Design
│   ├── Step 1.1: Define user schema
│   ├── Step 1.2: Create migration
│   └── Step 1.3: Add indexes
├── Milestone 2: API Endpoints
│   ├── Step 2.1: Auth middleware
│   ├── Step 2.2: CRUD routes
│   └── Step 2.3: Validation
└── Milestone 3: Testing
    ├── Step 3.1: Unit tests
    ├── Step 3.2: Integration tests
    └── Step 3.3: Load tests
```

**Impact:** +24% on SWE-Bench (CMU, 2025)

---

### Phase 3: FLARE Lookahead Planning

**File:** `src/agent/lookahead.rs` (~300 lines)

**Core Algorithm:**
1. Generate 2-3 candidate plans
2. For each plan, simulate execution
3. Evaluate counterfactual outcomes
4. Propagate outcomes backward
5. Select plan with best projected result

**Lookahead Depth:** 2-3 steps (configurable)

**Evaluation Criteria:**
- Success probability
- Time estimate accuracy
- Risk exposure
- Resource requirements

**Impact:** LLaMA-8B with FLARE beats GPT-4o (Wang et al., 2026)

---

### Phase 4: Memory System

**File:** `src/agent/memory.rs` (~400 lines)

**Memory Types:**

| Type | Storage | TTL | Purpose |
|------|---------|-----|---------|
| Semantic | JSON file | Permanent | Facts and knowledge |
| Episodic | JSON file | Permanent | Past experiences |
| Procedural | JSON file | Permanent | Successful patterns |
| Working | In-memory | Session | Current plan state |

**Memory Operations:**
- `store(key, value, type)` — Save to memory
- `recall(query)` — Retrieve relevant memories
- `consolidate()` — Merge related memories
- `decay()` — Remove irrelevant memories

**Storage Format:**
```json
{
  "memories": [
    {
      "id": "mem_001",
      "type": "episodic",
      "content": "API auth middleware took 2 hours due to JWT edge cases",
      "tags": ["auth", "jwt", "time-estimate"],
      "confidence": 0.9,
      "created_at": "2026-06-30",
      "last_accessed": "2026-06-30",
      "access_count": 3
    }
  ]
}
```

---

### Phase 5: Dev Workflow Automation

**File:** `src/agent/workflow.rs` (~500 lines)

**6 Automated Workflows:**

#### 5a. Code Generation
1. Decompose feature into components
2. Generate code for each component
3. CID gates validate quality
4. Generate tests for each component
5. Self-review with CID scoring
6. Iterate on issues

**Impact:** +90% on boilerplate, +70% on tests

#### 5b. Debugging
1. Reproduce: Create minimal failing test
2. Hypothesize: Generate 3-5 possible causes
3. Verify: Check each hypothesis against code
4. Fix: Implement fix for confirmed cause
5. Validate: Verify fix doesn't break other tests
6. Document: Record lesson in memory

**Impact:** +20-40% on shallow bugs

#### 5c. Documentation
1. Analyze: Understand code structure
2. Outline: Generate documentation outline
3. Write: Generate detailed documentation
4. Validate: Check accuracy against code
5. Format: Apply consistent formatting
6. Store: Add to knowledge base

**Impact:** +65% on documentation tasks

#### 5d. Test Generation
1. Analyze: Understand code to test
2. Plan: Design test strategy
3. Generate: Create test cases
4. Validate: Run tests, check coverage
5. Enhance: Add edge cases
6. Document: Add test descriptions

**Impact:** +70% on test generation

#### 5e. Code Review
1. Analyze: Understand changed code
2. Check: Run CID gates on changes
3. Review: Identify issues and improvements
4. Suggest: Provide actionable feedback
5. Verify: Check suggestions are correct
6. Report: Generate review summary

**Impact:** +30-55% on code review

#### 5f. Refactoring
1. Analyze: Identify code smells
2. Plan: Design refactoring approach
3. Execute: Make changes incrementally
4. Validate: Run tests after each change
5. Verify: Check no regressions
6. Document: Record refactoring patterns

**Impact:** +52.5% reduction in code smells

---

### Phase 6: Thought Exploration (ToT/GoT)

**File:** `src/agent/thoughts.rs` (~350 lines)

**Tree of Thoughts (ToT):**
- Generate 2-3 reasoning paths
- Evaluate each with CID scoring
- Backtrack when paths are unpromising
- Best-first search over solution space

**Graph of Thoughts (GoT):**
- Allow combining/merging thoughts
- Enable feedback loops
- +62% over ToT, -31% cost

**Adaptive Exploration:**
- Start broad, narrow based on scores
- Focus compute on promising paths
- Configurable exploration depth

**Impact:** 4% → 74% on complex problems (ToT)

---

### Phase 7: Self-Consistency for Plans

**File:** `src/agent/consistency.rs` (~250 lines)

**Algorithm:**
1. Generate N plans (default 5)
2. Each plan gets CID validation
3. Extract key decisions from each
4. Confidence-weighted voting
5. Select plan with highest weighted score

**Adaptive Sampling:**
- Start with 3 plans
- If agreement > 90%, stop
- If agreement < 60%, generate 2 more
- Max 8 plans

**Impact:** +15-25% on reasoning tasks

---

## 4. File Structure

```
src/
├── agent/
│   ├── mod.rs          (~50 lines)   Module root
│   ├── planner.rs      (~400 lines)  Goal tracking, plan generation
│   ├── decompose.rs    (~350 lines)  Task decomposition
│   ├── lookahead.rs    (~300 lines)  FLARE lookahead planning
│   ├── memory.rs       (~400 lines)  Memory system
│   ├── workflow.rs     (~500 lines)  Dev workflow automation
│   ├── thoughts.rs     (~350 lines)  ToT/GoT exploration
│   └── consistency.rs  (~250 lines)  Self-consistency voting
```

**Total new code:** ~2,600 lines across 8 files.

---

## 5. Integration Points

### With Existing CID Gates
- Plan validation uses math, logic, fact, confidence gates
- Code validation uses fallacy, bias, sanity gates
- Quality scoring uses response scorer

### With Existing MCP Tools
- `cid_validate` — Validate plan steps
- `cid_fix` — Fix issues found during validation
- `cid_lookup` — Retrieve facts for plan context
- `cid_search` — Search knowledge base
- `cid_score` — Score plan quality

### With Existing Infrastructure
- `Pipeline` — Validation pipeline
- `InferenceEngine` — LLM inference
- `KnowledgeBase` — Fact storage
- `BallEconomy` — Resource management
- `Budget` — Cost tracking

---

## 6. Expected Results

| Metric | Current | Target | Improvement |
|--------|---------|--------|-------------|
| LLM quality boost | 48% | 60-80% | +25-67% |
| Dev time savings | 3.6 hrs/week | 11.4 hrs/week | +217% |
| Task completion rate | 60-70% | 80-90% | +20-30% |
| Planning time | 30-60 min | 5-15 min | -75% |
| Bug introduction | 15-20% | 5-10% | -50% |
| Code quality | 65-75% | 85-95% | +25% |
| Documentation coverage | 30-50% | 80-90% | +80% |

---

## 7. Implementation Order

| Phase | Feature | Effort | Impact | Dependencies |
|-------|---------|--------|--------|--------------|
| 1 | Goal Tracker & Plan Generator | Medium | Foundation | None |
| 2 | Task Decomposition Engine | Medium | +24% task completion | Phase 1 |
| 3 | FLARE Lookahead | Medium | +10-15% accuracy | Phase 1 |
| 4 | Memory System | Large | Persistent knowledge | None |
| 5 | Dev Workflow Automation | Large | +217% dev time savings | Phases 1-4 |
| 6 | Thought Exploration | Large | +62% on complex problems | Phase 1 |
| 7 | Self-Consistency | Medium | +8-12% quality | Phase 1 |

**Recommended Start:** Phases 1 + 4 (Goal Tracker + Memory) — foundation for everything.

---

## 8. Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Plan generation too slow | High | Cache common plan patterns |
| Memory grows unbounded | Medium | Implement decay and consolidation |
| LLM calls too expensive | High | Use cheapest model for planning |
| Plans too rigid | Medium | Allow replanning on failure |
| Memory corruption | High | Use atomic writes, backups |

---

## 9. Success Criteria

| Criterion | Target | Measurement |
|-----------|--------|-------------|
| Plan generation works | 90% success rate | Test on 100 goals |
| Decomposition quality | +20% task completion | Compare with/without |
| Memory recall relevant | 80% precision | Manual evaluation |
| Workflow automation saves time | 11+ hrs/week | Developer survey |
| Code quality improves | +25% score | CID scoring |

---

## 10. Future Enhancements

| Enhancement | Description | Priority |
|-------------|-------------|----------|
| Multi-project planning | Plans across multiple codebases | High |
| Team coordination | Shared plans and memory | Medium |
| Learning from failures | Auto-improve plans based on outcomes | High |
| Real-time adaptation | Adjust plans during execution | Medium |
| Cross-language support | Plans for polyglot projects | Low |

---

## Appendix: Research Sources

| Paper/Source | Year | Key Finding |
|-------------|------|-------------|
| Wang et al. (FLARE) | 2026 | LLaMA-8B beats GPT-4o with lookahead |
| CMU Task Decomposition | 2025 | +24% on SWE-Bench |
| Yao et al. (ToT) | 2023 | 4% → 74% on Game of 24 |
| Besta et al. (GoT) | 2024 | +62% over ToT, -31% cost |
| Ivern AI Survey | 2026 | 11.4 hrs/week saved |
| Yao et al. (ReAct) | 2022 | 6% hallucination vs 56% CoT |
| Wang et al. (SC) | 2022 | +5-15pp accuracy |
| Meta AI (CoVe) | 2024 | +40% on technical writing |
| ACONIC Framework | 2026 | +10-40pp on SAT-Bench |
| HiPlan | 2025 | Hierarchical planning |
| MAP Framework | 2025 | Brain-inspired planning |
| Pre-Act | 2025 | +70% Action Recall vs ReAct |

---

**Document Version:** 1.0
**Last Updated:** 2026-06-30
**Status:** Ready for Implementation
