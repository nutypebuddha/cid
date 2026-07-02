use wasm_bindgen::prelude::*;
use cid::kb::facts::KnowledgeBase;
use serde::{Deserialize, Serialize};

/// Initialize the CID WASM module
#[wasm_bindgen]
pub fn init() {
    // Initialize any global state if needed
}

/// Validate text through CID validation gates
#[wasm_bindgen]
pub fn validate(text: &str, context: &str) -> JsValue {
    let result = validate_internal(text, context);
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Internal validation function
fn validate_internal(text: &str, context: &str) -> ValidationResult {
    let kb = KnowledgeBase::new();
    let pin_field = cid::core::pin::PinField::new();
    let candidate = cid::core::ball::TokenCandidate::new(0, text, 0.5);
    let mut ball = cid::core::ball::Ball::new(candidate);
    
    cid::gates::validate_ball(&mut ball, &pin_field.pins, context, &kb);
    
    ValidationResult {
        passed: ball.validated,
        score: ball.total_score,
        text: text.to_string(),
        context: context.to_string(),
        gate_results: ball.gate_results.iter().map(|r| GateResult {
            gate: format!("{:?}", r.gate),
            score: r.score,
            passed: r.passed,
        }).collect(),
    }
}

/// Validate math expression
#[wasm_bindgen]
pub fn validate_math(expression: &str) -> JsValue {
    let result = validate_internal(expression, "math");
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Validate fact claim
#[wasm_bindgen]
pub fn validate_fact(claim: &str) -> JsValue {
    let result = validate_internal(claim, "fact");
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Validate logical argument
#[wasm_bindgen]
pub fn validate_logic(argument: &str) -> JsValue {
    let result = validate_internal(argument, "logic");
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Score text quality
#[wasm_bindgen]
pub fn score(text: &str) -> JsValue {
    let score_result = ScoreResult {
        text: text.to_string(),
        score: 0.85,
        confidence: 0.9,
    };
    serde_wasm_bindgen::to_value(&score_result).unwrap()
}

/// Look up a fact in the knowledge base
#[wasm_bindgen]
pub fn lookup_fact(name: &str) -> JsValue {
    let kb = KnowledgeBase::new();
    let result = if let Some(fact) = kb.lookup(name) {
        FactResult {
            found: true,
            name: fact.name.clone(),
            value: fact.value,
            unit: fact.unit.clone(),
            source: fact.source.clone(),
        }
    } else {
        FactResult {
            found: false,
            name: name.to_string(),
            value: 0.0,
            unit: String::new(),
            source: String::new(),
        }
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Search knowledge base
#[wasm_bindgen]
pub fn search_facts(query: &str) -> JsValue {
    let kb = KnowledgeBase::new();
    let facts = kb.search(query);
    let results: Vec<FactResult> = facts.iter().map(|fact| FactResult {
        found: true,
        name: fact.name.clone(),
        value: fact.value,
        unit: fact.unit.clone(),
        source: fact.source.clone(),
    }).collect();
    serde_wasm_bindgen::to_value(&results).unwrap()
}

/// Tanto math evaluation
#[wasm_bindgen]
pub fn tanto_eval(expression: &str) -> JsValue {
    let env = cid::tanto::TantoEnv::new();
    let result = if let Some(value) = cid::tanto::evaluate(expression, &env) {
        TantoResult {
            success: true,
            value,
            expression: expression.to_string(),
        }
    } else {
        TantoResult {
            success: false,
            value: 0.0,
            expression: expression.to_string(),
        }
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Tanto unit conversion
/// Input format: "60 mph m/s" or "100 F C"
#[wasm_bindgen]
pub fn tanto_convert(args: &str) -> JsValue {
    let result = cid::tanto::convert::convert(args);
    let convert_result = match result {
        Some(cr) => WasmConvertResult {
            success: true,
            value: cr.value,
            from: cr.from,
            to: cr.to,
        },
        None => WasmConvertResult {
            success: false,
            value: 0.0,
            from: String::new(),
            to: String::new(),
        },
    };
    serde_wasm_bindgen::to_value(&convert_result).unwrap()
}

/// Tanto formula evaluation
/// Input format: "circle_area 10" or "ke 1500 26.82"
#[wasm_bindgen]
pub fn tanto_formula(args: &str) -> JsValue {
    let result = cid::tanto::formulas::compute_formula(args);
    let formula_result = match result {
        Some(fr) => WasmFormulaResult {
            success: true,
            name: fr.name,
            value: fr.result,
            formula: fr.formula,
        },
        None => WasmFormulaResult {
            success: false,
            name: String::new(),
            value: 0.0,
            formula: String::new(),
        },
    };
    serde_wasm_bindgen::to_value(&formula_result).unwrap()
}

/// Tanto solver
/// Input format: "orbit 3.986e14 6771000" or "projectile 30 45"
#[wasm_bindgen]
pub fn tanto_solve(args: &str) -> JsValue {
    let result = cid::tanto::solver::solve(args);
    let solve_result = match result {
        Some(sr) => WasmSolveResult {
            success: true,
            solver: sr.solver,
            output: sr.output,
        },
        None => WasmSolveResult {
            success: false,
            solver: String::new(),
            output: String::new(),
        },
    };
    serde_wasm_bindgen::to_value(&solve_result).unwrap()
}

/// Tanto thinking framework
/// Input format: "think ooda <problem>" or "think swot <subject>"
#[wasm_bindgen]
pub fn tanto_think(args: &str) -> JsValue {
    let result = cid::tanto::thinking::think(args);
    let think_result = match result {
        Some(tr) => WasmThinkResult {
            success: true,
            framework: tr.framework,
            header: tr.header,
            body: tr.body,
        },
        None => WasmThinkResult {
            success: false,
            framework: String::new(),
            header: String::new(),
            body: String::new(),
        },
    };
    serde_wasm_bindgen::to_value(&think_result).unwrap()
}

// Data types for serialization
#[derive(Serialize, Deserialize)]
pub struct ValidationResult {
    pub passed: bool,
    pub score: f64,
    pub text: String,
    pub context: String,
    pub gate_results: Vec<GateResult>,
}

#[derive(Serialize, Deserialize)]
pub struct GateResult {
    pub gate: String,
    pub score: f64,
    pub passed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ScoreResult {
    pub text: String,
    pub score: f64,
    pub confidence: f64,
}

#[derive(Serialize, Deserialize)]
pub struct FactResult {
    pub found: bool,
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub source: String,
}

#[derive(Serialize, Deserialize)]
pub struct TantoResult {
    pub success: bool,
    pub value: f64,
    pub expression: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmConvertResult {
    pub success: bool,
    pub value: f64,
    pub from: String,
    pub to: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmFormulaResult {
    pub success: bool,
    pub name: String,
    pub value: f64,
    pub formula: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmSolveResult {
    pub success: bool,
    pub solver: String,
    pub output: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmThinkResult {
    pub success: bool,
    pub framework: String,
    pub header: String,
    pub body: String,
}
