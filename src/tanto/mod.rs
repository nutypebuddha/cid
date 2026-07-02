// Tanto compute module — merged into CID
// Provides: math expressions, unit conversion, physics formulas,
// multi-step solvers, knowledge base, sanity checks, thinking frameworks,
// pipeline evaluation, and self-verification.

pub mod math;
pub mod parser;
pub mod rational;
pub mod determinism;
pub mod convert;
pub mod formulas;
pub mod solver;
pub mod sanity;
pub mod thinking;
pub mod pipeline;
pub mod verify;

use std::collections::HashMap;

/// Tanto environment: variables, trace, and knowledge base
pub struct TantoEnv {
    pub vars: HashMap<String, f64>,
    pub ans: Option<f64>,
}

impl TantoEnv {
    pub fn new() -> Self {
        TantoEnv {
            vars: HashMap::new(),
            ans: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        if name == "ans" {
            return self.ans;
        }
        // Check built-in constants
        if let Some(val) = get_constant(name) {
            return Some(val);
        }
        self.vars.get(name).copied()
    }

    pub fn set(&mut self, name: &str, val: f64) {
        self.vars.insert(name.to_string(), val);
    }

    pub fn set_ans(&mut self, val: f64) {
        self.ans = Some(val);
    }
}

/// Built-in physical constants matching Tanto's values
pub fn get_constant(name: &str) -> Option<f64> {
    match name {
        "pi" => Some(3.141592653589793),
        "e" => Some(2.718281828459045),
        "c" => Some(299792458.0),
        "c_squared" => Some(89875517873681764.0),
        "R_air" => Some(287.05869491149255),
        "g" => Some(9.80665),
        "G" => Some(6.67430e-11),
        "h" => Some(6.62607015e-34),
        "hbar" => Some(1.054571817e-34),
        "kB" => Some(1.380649e-23),
        "e_charge" => Some(1.602176634e-19),
        "me" => Some(9.1093837015e-31),
        "mp" => Some(1.67262192369e-27),
        "NA" => Some(6.02214076e23),
        "R" => Some(8.314462618),
        "atm" => Some(101325.0),
        "Rearth" => Some(6371000.0),
        "GMearth" => Some(3.986004418e14),
        _ => None,
    }
}

/// Evaluate a math expression string using Tanto's parser.
/// Tries exact rational arithmetic first, then falls back to f64.
pub fn evaluate(expr: &str, env: &TantoEnv) -> Option<f64> {
    // Try exact rational arithmetic first (for fraction expressions like 1/3 + 1/6)
    if let Some(rat) = rational::eval_rational(expr, env) {
        return Some(rat.to_f64());
    }
    parser::eval_math(expr.as_bytes(), env)
        .or_else(|| parser::eval_op_format(expr.as_bytes(), env))
}

/// Evaluate with natural language preprocessing
pub fn evaluate_nl(expr: &str, env: &TantoEnv) -> Option<f64> {
    let processed = natural::preprocess_line(expr.as_bytes());
    let processed = std::str::from_utf8(&processed).unwrap_or(expr);
    evaluate(processed, env)
}

/// Natural language preprocessing (inline, adapted from Tanto)
pub mod natural {
    pub fn preprocess_line(input: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(input.len());

        // Check for "X% of Y" pattern
        if let Some((pct, rest)) = parse_pct_of(input) {
            let rest = strip_fillers(rest);
            result.extend_from_slice(rest);
            result.extend_from_slice(b" * ");
            result.extend_from_slice(pct);
            return result;
        }

        let stripped = strip_fillers(input);

        let end = stripped.len();
        let mut new_end = end;
        while new_end > 0 && matches!(stripped[new_end - 1], b'?' | b'.' | b'!' | b',') {
            new_end -= 1;
        }
        result.extend_from_slice(&stripped[..new_end]);
        result
    }

    fn strip_fillers(input: &[u8]) -> &[u8] {
        let fillers: [&[u8]; 12] = [
            b"whats ", b"what's ", b"calculate ", b"compute ",
            b"please ", b"can you ", b"tell me ", b"give me ",
            b"what is ", b"what are ", b"how much is ", b"how many ",
        ];
        for filler in &fillers {
            if input.len() >= filler.len() && &input[..filler.len()] == *filler {
                return &input[filler.len()..];
            }
        }
        input
    }

    fn parse_pct_of(input: &[u8]) -> Option<(&[u8], &[u8])> {
        let of_pos = find_subsequence(input, b" of ")?;
        let pct_part = &input[..of_pos];
        let rest = &input[of_pos + 4..];
        if pct_part.is_empty() || pct_part[pct_part.len() - 1] != b'%' { return None; }
        if pct_part[0] < b'0' || pct_part[0] > b'9' { return None; }
        Some((pct_part, rest))
    }

    fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        if needle.is_empty() || needle.len() > haystack.len() { return None; }
        for i in 0..=haystack.len() - needle.len() {
            if &haystack[i..i + needle.len()] == needle { return Some(i); }
        }
        None
    }
}
