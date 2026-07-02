// Tanto determinism audit — verifies every operation produces bit-identical results
// across multiple runs. This is critical for a deterministic compute engine.
//
// Each test runs the operation N times (default 100) and asserts that every run
// produces the exact same f64 bits. Uses f64::to_bits() for comparison to catch
// even the smallest floating-point variation.

use crate::tanto::TantoEnv;

/// Default iterations per test
const ITERATIONS: usize = 100;

/// Result of a single determinism check
#[derive(Debug, Clone)]
pub struct DeterminismResult {
    pub description: &'static str,
    pub passed: bool,
    pub variations: usize, // number of non-identical results (0 = perfect)
    pub sample_value: String,
}

/// Run the full determinism audit.
/// Returns a list of (description, passed) tuples matching verify::run_self_test format.
pub fn run_determinism_audit() -> Vec<(&'static str, bool)> {
    let mut results = vec![];
    let mut all_pass = true;
    let mut total = 0;
    let mut passed = 0;

    // Macro to add tests more cleanly
    let mut add = |desc: &'static str, ok: bool| {
        total += 1;
        if ok { passed += 1; } else { all_pass = false; }
        results.push((desc, ok));
    };

    // ─── Basic Arithmetic ───────
    add("arithmetic: 2+3", check_determinism("2+3", None));
    add("arithmetic: 10/3", check_determinism("10/3", None));
    add("arithmetic: 1.5*2.5", check_determinism("1.5*2.5", None));
    add("arithmetic: 100-37", check_determinism("100-37", None));
    add("arithmetic: (2+3)*(4-1)", check_determinism("(2+3)*(4-1)", None));
    add("arithmetic: 2^10", check_determinism("2^10", None));
    add("arithmetic: -42", check_determinism("-42", None));
    add("arithmetic: +42", check_determinism("+42", None));
    add("arithmetic: 7%3", check_determinism("7%3", None));
    add("arithmetic: 1e5", check_determinism("1e5", None));
    add("arithmetic: 1.23e-4", check_determinism("1.23e-4", None));

    // ─── Functions ────────────
    add("func: sqrt(144)", check_determinism("sqrt(144)", None));
    add("func: sqrt(2)", check_determinism("sqrt(2)", None));
    add("func: sin(pi/2)", check_determinism("sin(pi/2)", None));
    add("func: cos(0)", check_determinism("cos(0)", None));
    add("func: tan(pi/4)", check_determinism("tan(pi/4)", None));
    add("func: asin(1)", check_determinism("asin(1)", None));
    add("func: acos(0.5)", check_determinism("acos(0.5)", None));
    add("func: atan(1)", check_determinism("atan(1)", None));
    add("func: hypot(3,4)", check_determinism("hypot(3,4)", None));
    add("func: abs(-5)", check_determinism("abs(-5)", None));
    add("func: exp(1)", check_determinism("exp(1)", None));
    add("func: ln(e)", check_determinism("ln(e)", None));
    add("func: log10(100)", check_determinism("log10(100)", None));
    add("func: log2(8)", check_determinism("log2(8)", None));
    add("func: pow(2,10)", check_determinism("pow(2,10)", None));
    add("func: round(1.5)", check_determinism("round(1.5)", None));
    add("func: floor(2.7)", check_determinism("floor(2.7)", None));
    add("func: ceil(2.3)", check_determinism("ceil(2.3)", None));
    add("func: min(1,2,3)", check_determinism("min(1,2,3)", None));
    add("func: max(1,2,3)", check_determinism("max(1,2,3)", None));
    add("func: clamp(5,0,10)", check_determinism("clamp(5,0,10)", None));
    add("func: sum(1,2,3,4)", check_determinism("sum(1,2,3,4)", None));
    add("func: avg(10,20,30)", check_determinism("avg(10,20,30)", None));
    add("func: rad2deg(pi)", check_determinism("rad2deg(pi)", None));
    add("func: deg2rad(180)", check_determinism("deg2rad(180)", None));
    add("func: atan2(0,1)", check_determinism("atan2(0,1)", None));
    add("func: norm(3,4)", check_determinism("norm(3,4)", None));

    // ─── Named Ops ─────────────
    add("named: add 2 3", check_determinism("add 2 3", None));
    add("named: sub 10 4", check_determinism("sub 10 4", None));
    add("named: mul 3 4", check_determinism("mul 3 4", None));
    add("named: div 12 4", check_determinism("div 12 4", None));
    add("named: neg 5", check_determinism("neg 5", None));
    add("named: pct 200 15", check_determinism("pct 200 15", None));
    add("named: pct_change 100 150", check_determinism("pct_change 100 150", None));
    add("named: ratio 4 3 2", check_determinism("ratio 4 3 2", None));
    add("named: dilute 10 5", check_determinism("dilute 10 5", None));

    // ─── Unit Conversions ──────
    add("convert: mi_to_km(10)", check_determinism("mi_to_km(10)", None));
    add("convert: km_to_mi(16.09344)", check_determinism("km_to_mi(16.09344)", None));
    add("convert: mph_to_kmh(60)", check_determinism("mph_to_kmh(60)", None));
    add("convert: kmh_to_mph(100)", check_determinism("kmh_to_mph(100)", None));
    add("convert: f_to_c(212)", check_determinism("f_to_c(212)", None));
    add("convert: c_to_f(0)", check_determinism("c_to_f(0)", None));
    add("convert: c_to_k(0)", check_determinism("c_to_k(0)", None));
    add("convert: lb_to_kg(100)", check_determinism("lb_to_kg(100)", None));
    add("convert: kg_to_lb(45.3592)", check_determinism("kg_to_lb(45.3592)", None));
    add("convert: ft_to_m(10)", check_determinism("ft_to_m(10)", None));
    add("convert: in_to_cm(12)", check_determinism("in_to_cm(12)", None));
    add("convert: mph_to_ms(60)", check_determinism("mph_to_ms(60)", None));
    add("convert: ms_to_mph(26.8224)", check_determinism("ms_to_mph(26.8224)", None));

    // ─── Rational Arithmetic ───
    add("rational: 1/3 + 1/6", check_determinism_rational("1/3 + 1/6"));
    add("rational: 1/3 * 3", check_determinism_rational("1/3 * 3"));
    add("rational: (1+2)/3", check_determinism_rational("(1+2)/3"));
    add("rational: 1 + 2/3", check_determinism_rational("1 + 2/3"));
    add("rational: 5/7 + 2/7", check_determinism_rational("5/7 + 2/7"));
    add("rational: 10/3 - 1/3", check_determinism_rational("10/3 - 1/3"));

    // ─── Constants ────────────
    add("const: pi", check_determinism("pi", None));
    add("const: e", check_determinism("e", None));
    add("const: c (speed of light)", check_determinism("c", None));
    add("const: g (gravity)", check_determinism("g", None));
    add("const: G (gravitational)", check_determinism("G", None));
    add("const: h (planck)", check_determinism("h", None));
    add("const: kB", check_determinism("kB", None));

    // ─── Natural Language ─────
    add("nl: 15% of 240", check_determinism_nl("15% of 240"));
    add("nl: 10% of 250", check_determinism_nl("10% of 250"));
    add("nl: whats sqrt(144)", check_determinism_nl("whats sqrt(144)"));
    add("nl: calculate 2+3", check_determinism_nl("calculate 2+3"));
    add("nl: how much is 42", check_determinism_nl("how much is 42"));

    // ─── Formulas ─────────────
    add("formula: circle_area(5)", check_determinism_formula("circle_area", "5"));
    add("formula: sphere_volume(3)", check_determinism_formula("sphere_volume", "3"));
    add("formula: ke(10,5)", check_determinism_formula("ke", "10 5"));
    add("formula: pe(5,9.8)", check_determinism_formula("pe", "5 9.8"));
    add("formula: force(10,5)", check_determinism_formula("force", "10 5"));
    add("formula: pressure(100,2)", check_determinism_formula("pressure", "100 2"));
    add("formula: work(10,5)", check_determinism_formula("work", "10 5"));
    add("formula: power(100,10)", check_determinism_formula("power", "100 10"));

    // ─── Solvers ──────────────
    add("solver: stats 1 2 3 4 5", check_determinism_solver("stats", "1 2 3 4 5"));
    add("solver: compound 1000 0.05 12 10", check_determinism_solver("compound", "1000 0.05 12 10"));
    add("solver: growth 100 0.1 5", check_determinism_solver("growth", "100 0.1 5"));

    // ─── Pipeline ─────────────
    add("pipeline: div 1 3 | mul 6 _", check_determinism_pipeline("div 1 3 | mul 6 _"));
    add("pipeline: add 2 3 | mul 4 _", check_determinism_pipeline("add 2 3 | mul 4 _"));

    // ─── Chained / Complex ────
    add("chained: 1+2*3+4*5", check_determinism("1+2*3+4*5", None));
    add("chained: sqrt(3^2 + 4^2)", check_determinism("sqrt(3^2 + 4^2)", None));
    add("chained: sin(pi/6)^2 + cos(pi/6)^2", check_determinism("sin(pi/6)^2 + cos(pi/6)^2", None));
    add("chained: ln(e^5)", check_determinism("ln(e^5)", None));
    add("chained: 10% of 500 + 5% of 200", check_determinism_nl("10% of 500 + 5% of 200"));

    results
}

/// Check that an f64 expression produces identical results across ITERATIONS runs.
pub fn check_determinism(expr: &str, expected: Option<f64>) -> bool {
    let env = TantoEnv::new();
    let first = match crate::tanto::evaluate_nl(expr, &env) {
        Some(v) => v,
        None => return false,
    };

    // Check expected value if provided
    if let Some(exp) = expected {
        if (first - exp).abs() > 1e-6 {
            return false;
        }
    }

    // Run ITERATIONS times
    for _ in 1..ITERATIONS {
        let env = TantoEnv::new();
        match crate::tanto::evaluate_nl(expr, &env) {
            Some(v) => {
                if v.to_bits() != first.to_bits() {
                    return false; // Non-deterministic!
                }
            }
            None => return false,
        }
    }
    true
}

/// Check determinism of rational evaluation
pub fn check_determinism_rational(expr: &str) -> bool {
    let env = TantoEnv::new();
    let first = match crate::tanto::rational::eval_rational(expr, &env) {
        Some(r) => r,
        None => return false,
    };

    for _ in 1..ITERATIONS {
        let env = TantoEnv::new();
        match crate::tanto::rational::eval_rational(expr, &env) {
            Some(r) => {
                if r != first {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

/// Check determinism of natural language processing
pub fn check_determinism_nl(expr: &str) -> bool {
    let env = TantoEnv::new();
    let first = match crate::tanto::evaluate_nl(expr, &env) {
        Some(v) => v,
        None => return false,
    };

    for _ in 1..ITERATIONS {
        let env = TantoEnv::new();
        match crate::tanto::evaluate_nl(expr, &env) {
            Some(v) => {
                if v.to_bits() != first.to_bits() {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

/// Check determinism of formula computation
pub fn check_determinism_formula(name: &str, args: &str) -> bool {
    let line = format!("{} {}", name, args);
    let first = match crate::tanto::formulas::compute_formula(&line) {
        Some(r) => r.result,
        None => return false,
    };

    for _ in 1..ITERATIONS {
        let line = format!("{} {}", name, args);
        match crate::tanto::formulas::compute_formula(&line) {
            Some(r) => {
                if r.result.to_bits() != first.to_bits() {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

/// Check determinism of solver computation
pub fn check_determinism_solver(name: &str, args: &str) -> bool {
    let line = format!("{} {}", name, args);
    let first = match crate::tanto::solver::solve(&line) {
        Some(r) => r.output.clone(),
        None => return false,
    };

    for _ in 1..ITERATIONS {
        let line = format!("{} {}", name, args);
        match crate::tanto::solver::solve(&line) {
            Some(r) => {
                if r.output != first {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

/// Check determinism of pipeline evaluation
pub fn check_determinism_pipeline(expr: &str) -> bool {
    let env = TantoEnv::new();
    let first = match crate::tanto::pipeline::evaluate_pipeline(expr, &env) {
        Some(v) => v,
        None => return false,
    };

    for _ in 1..ITERATIONS {
        let env = TantoEnv::new();
        match crate::tanto::pipeline::evaluate_pipeline(expr, &env) {
            Some(v) => {
                if v.to_bits() != first.to_bits() {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinism_basic_arithmetic() {
        assert!(check_determinism("2+3", None));
        assert!(check_determinism("10/3", None));
        assert!(check_determinism("1.5*2.5", None));
    }

    #[test]
    fn test_determinism_with_expected() {
        assert!(check_determinism("2+3", Some(5.0)));
        assert!(check_determinism("sqrt(144)", Some(12.0)));
    }

    #[test]
    fn test_determinism_rational() {
        assert!(check_determinism_rational("1/3 + 1/6"));
        assert!(check_determinism_rational("1/3 * 3"));
    }

    #[test]
    fn test_determinism_nl() {
        assert!(check_determinism_nl("15% of 240"));
    }

    #[test]
    fn test_determinism_formula() {
        assert!(check_determinism_formula("circle_area", "5"));
        assert!(check_determinism_formula("sphere_volume", "3"));
    }

    #[test]
    fn test_determinism_solver() {
        assert!(check_determinism_solver("stats", "1 2 3 4 5"));
    }

    #[test]
    fn test_determinism_pipeline() {
        assert!(check_determinism_pipeline("div 1 3 | mul 6 _"));
    }

    #[test]
    fn test_determinism_audit_all_pass() {
        let results = run_determinism_audit();
        let failures: Vec<_> = results.iter().filter(|(_, ok)| !ok).collect();
        assert!(failures.is_empty(),
            "Determinism audit failed on {} tests:\n{}",
            failures.len(),
            failures.iter().map(|(desc, _)| format!("  FAIL: {}", desc)).collect::<Vec<_>>().join("\n")
        );
    }
}
