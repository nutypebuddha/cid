// Tanto exact rational arithmetic — `Rational(i64, i64)` with full arithmetic.
// Used internally by evaluate() to produce exact results for fraction expressions,
// preventing floating-point drift in chained operations.
//
// The parser handles: integers, + - * /, parentheses, and variables from env.
// Non-rational constructs (trig, sqrt, decimals) return None → fallback to f64 parser.

use crate::tanto::TantoEnv;

/// An exact rational number: numerator / denominator (always reduced, denominator > 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational(i64, i64);

impl Rational {
    /// Create a new rational, reducing to lowest terms.
    /// Returns None if denominator is zero.
    pub fn new(n: i64, d: i64) -> Option<Self> {
        if d == 0 {
            return None;
        }
        let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
        let g = gcd(n.unsigned_abs(), d as u64) as i64;
        Some(Rational(n / g, d / g))
    }

    /// Zero: 0/1
    pub fn zero() -> Self {
        Rational(0, 1)
    }

    /// One: 1/1
    pub fn one() -> Self {
        Rational(1, 1)
    }

    /// Convert to f64 (inexact for many fractions)
    pub fn to_f64(&self) -> f64 {
        self.0 as f64 / self.1 as f64
    }

    /// Format as string: "1/3" or "5" (if denominator is 1)
    pub fn format(&self) -> String {
        if self.1 == 1 {
            format!("{}", self.0)
        } else {
            format!("{}/{}", self.0, self.1)
        }
    }

    /// Format as mixed number: "1 2/3" instead of "5/3"
    pub fn format_mixed(&self) -> String {
        if self.1 == 1 {
            return format!("{}", self.0);
        }
        let whole = self.0 / self.1;
        let rem = self.0.abs() % self.1;
        if whole == 0 {
            self.format()
        } else {
            format!("{} {}/{}", whole, rem, self.1)
        }
    }

    // ─── Arithmetic ────────────────────────────────────────────

    pub fn add(&self, other: &Self) -> Self {
        // a/b + c/d = (a*d + c*b) / (b*d)
        let n = self.0 * other.1 + other.0 * self.1;
        let d = self.1 * other.1;
        // Self::new reduces
        Self::new(n, d).unwrap_or(Rational::zero())
    }

    pub fn sub(&self, other: &Self) -> Self {
        let n = self.0 * other.1 - other.0 * self.1;
        let d = self.1 * other.1;
        Self::new(n, d).unwrap_or(Rational::zero())
    }

    pub fn mul(&self, other: &Self) -> Self {
        let n = self.0 * other.0;
        let d = self.1 * other.1;
        Self::new(n, d).unwrap_or(Rational::zero())
    }

    pub fn div(&self, other: &Self) -> Option<Self> {
        if other.0 == 0 {
            return None;
        }
        let n = self.0 * other.1;
        let d = self.1 * other.0;
        Self::new(n, d)
    }

    /// Negation
    pub fn neg(&self) -> Self {
        Rational(-self.0, self.1)
    }

    /// Absolute value
    pub fn abs(&self) -> Self {
        Rational(self.0.unsigned_abs() as i64, self.1)
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

// ─── GCD (binary GCD for speed) ───────────────────────────────

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let a_trailing = a.trailing_zeros();
    let b_trailing = b.trailing_zeros();
    let shift = a_trailing.min(b_trailing);
    let mut a = a >> a_trailing;
    let mut b = b >> b_trailing;
    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }
    a << shift
}

// ─── Rational Expression Parser ───────────────────────────────
// Recursive descent, same precedence rules as main parser:
//   + -  (lowest)
//   * /  (middle)
//   unary - (highest)
// Variables resolve via env (must be integer-convertible)

struct RatParser<'a> {
    bytes: &'a [u8],
    pos: usize,
    env: &'a TantoEnv,
}

impl<'a> RatParser<'a> {
    fn new(bytes: &'a [u8], env: &'a TantoEnv) -> Self {
        RatParser { bytes, pos: 0, env }
    }

    fn peek(&self) -> Option<u8> {
        if self.pos < self.bytes.len() {
            Some(self.bytes[self.pos])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<u8> {
        if self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            self.pos += 1;
            Some(b)
        } else {
            None
        }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn parse_expr(&mut self) -> Option<Rational> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Option<Rational> {
        let mut left = self.parse_mul_div()?;
        loop {
            self.skip_ws();
            match self.peek() {
                Some(b'+') => {
                    self.advance();
                    let right = self.parse_mul_div()?;
                    left = left.add(&right);
                }
                Some(b'-') => {
                    self.advance();
                    let right = self.parse_mul_div()?;
                    left = left.sub(&right);
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_mul_div(&mut self) -> Option<Rational> {
        let mut left = self.parse_unary()?;
        loop {
            self.skip_ws();
            match self.peek() {
                Some(b'*') => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = left.mul(&right);
                }
                Some(b'/') => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = left.div(&right)?;
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<Rational> {
        self.skip_ws();
        match self.peek() {
            Some(b'-') => {
                self.advance();
                let val = self.parse_primary()?;
                Some(val.neg())
            }
            Some(b'+') => {
                self.advance();
                self.parse_primary()
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Option<Rational> {
        self.skip_ws();
        match self.peek() {
            Some(b'(') => {
                self.advance();
                let val = self.parse_expr()?;
                self.skip_ws();
                if self.peek() == Some(b')') {
                    self.advance();
                    Some(val)
                } else {
                    None
                }
            }
            Some(c) if c.is_ascii_digit() => self.parse_integer(),
            Some(c) if c.is_ascii_alphabetic() || c == b'_' => self.parse_variable(),
            _ => None,
        }
    }

    fn parse_integer(&mut self) -> Option<Rational> {
        let start = self.pos;
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        let raw = &self.bytes[start..self.pos];
        let s = std::str::from_utf8(raw).ok()?;
        let n: i64 = s.parse().ok()?;
        Some(Rational(n, 1))
    }

    fn parse_variable(&mut self) -> Option<Rational> {
        let start = self.pos;
        while self.pos < self.bytes.len()
            && (self.bytes[self.pos].is_ascii_alphanumeric() || self.bytes[self.pos] == b'_')
        {
            self.pos += 1;
        }
        let name = std::str::from_utf8(&self.bytes[start..self.pos]).ok()?;

        // Check if it's a function call — not rational
        self.skip_ws();
        if self.peek() == Some(b'(') {
            return None; // Delegate to f64 parser for functions
        }

        // Look up variable
        if let Some(val) = self.env.get(name) {
            // Only accept variables that are exact integers (or very close)
            let rounded = val.round();
            if (val - rounded).abs() < 1e-12 {
                return Some(Rational(rounded as i64, 1));
            }
            // If non-integer, try rational conversion (e.g., 0.5 → 1/2)
            return rational_from_f64(val);
        }
        None
    }
}

/// Try to convert an f64 to an exact rational.
/// Handles: integers, halves, thirds, quarters, fifths, tenths, hundredths.
fn rational_from_f64(val: f64) -> Option<Rational> {
    if val.is_nan() || val.is_infinite() {
        return None;
    }
    if val == 0.0 {
        return Some(Rational::zero());
    }
    // Check for exact integer
    let rounded = val.round();
    if (val - rounded).abs() < 1e-12 {
        return Some(Rational(rounded as i64, 1));
    }
    // Common denominators: try denominators 1..1000
    let abs_val = val.abs();
    for d in 1..=1000 {
        let n = (abs_val * d as f64).round();
        let approx = n / d as f64;
        if (abs_val - approx).abs() < 1e-12 {
            let sign = if val < 0.0 { -1 } else { 1 };
            return Rational::new(sign * n as i64, d);
        }
    }
    None
}

/// Evaluate an expression as an exact rational.
/// Returns None if the expression contains non-rational elements (decimals, trig, etc.)
pub fn eval_rational(expr: &str, env: &TantoEnv) -> Option<Rational> {
    let expr = expr.trim();
    if expr.is_empty() {
        return None;
    }
    // Reject decimal points — rational parser only handles integers
    if expr.contains('.') {
        return None;
    }
    // Reject known non-rational functions
    let non_rational = [
        "sin(", "cos(", "tan(", "asin(", "acos(", "atan(", "atan2(",
        "sqrt(", "exp(", "ln(", "log(", "log10(", "log2(",
        "pow(", "hypot(", "round(", "floor(", "ceil(", "abs(",
    ];
    for &nr in &non_rational {
        if expr.contains(nr) {
            return None;
        }
    }
    let mut parser = RatParser::new(expr.as_bytes(), env);
    parser.parse_expr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rational_new() {
        let r = Rational::new(1, 3).unwrap();
        assert_eq!(r.0, 1);
        assert_eq!(r.1, 3);

        let r = Rational::new(2, 4).unwrap();
        assert_eq!(r.0, 1);
        assert_eq!(r.1, 2);

        let r = Rational::new(-2, 4).unwrap();
        assert_eq!(r.0, -1);
        assert_eq!(r.1, 2);

        let r = Rational::new(2, -4).unwrap();
        assert_eq!(r.0, -1);
        assert_eq!(r.1, 2);

        assert!(Rational::new(1, 0).is_none());
    }

    #[test]
    fn test_rational_arithmetic() {
        let a = Rational::new(1, 3).unwrap();
        let b = Rational::new(1, 6).unwrap();

        // 1/3 + 1/6 = 1/2
        assert_eq!(a.add(&b), Rational::new(1, 2).unwrap());

        // 1/3 - 1/6 = 1/6
        assert_eq!(a.sub(&b), Rational::new(1, 6).unwrap());

        // 1/3 * 1/6 = 1/18
        assert_eq!(a.mul(&b), Rational::new(1, 18).unwrap());

        // 1/3 / 1/6 = 2
        assert_eq!(a.div(&b), Some(Rational::new(2, 1).unwrap()));
    }

    #[test]
    fn test_rational_chain() {
        // 1/3 * 3 = 1  (no floating-point drift)
        let r = Rational::new(1, 3).unwrap();
        let three = Rational::new(3, 1).unwrap();
        assert_eq!(r.mul(&three), Rational::new(1, 1).unwrap());
    }

    #[test]
    fn test_rational_format() {
        assert_eq!(Rational::new(5, 1).unwrap().format(), "5");
        assert_eq!(Rational::new(1, 3).unwrap().format(), "1/3");
        assert_eq!(Rational::new(5, 3).unwrap().format_mixed(), "1 2/3");
    }

    #[test]
    fn test_rat_parser_simple() {
        let env = TantoEnv::new();
        assert_eq!(eval_rational("1/3", &env), Some(Rational::new(1, 3).unwrap()));
        assert_eq!(eval_rational("2+3", &env), Some(Rational::new(5, 1).unwrap()));
        assert_eq!(eval_rational("2*3", &env), Some(Rational::new(6, 1).unwrap()));
    }

    #[test]
    fn test_rat_parser_order() {
        let env = TantoEnv::new();
        // 1/3 + 1/6 = 1/2
        assert_eq!(eval_rational("1/3 + 1/6", &env), Some(Rational::new(1, 2).unwrap()));
        // 2 + 3 * 4 = 14 (not 20)
        assert_eq!(eval_rational("2 + 3 * 4", &env), Some(Rational::new(14, 1).unwrap()));
    }

    #[test]
    fn test_rat_parser_parentheses() {
        let env = TantoEnv::new();
        assert_eq!(eval_rational("(1 + 2) / 3", &env), Some(Rational::new(1, 1).unwrap()));
    }

    #[test]
    fn test_rat_parser_decimal_rejected() {
        let env = TantoEnv::new();
        assert!(eval_rational("1.5 + 2", &env).is_none());
    }

    #[test]
    fn test_rat_parser_trig_rejected() {
        let env = TantoEnv::new();
        assert!(eval_rational("sin(1/3)", &env).is_none());
    }

    #[test]
    fn test_rational_to_f64() {
        assert!((Rational::new(1, 2).unwrap().to_f64() - 0.5).abs() < 1e-15);
        assert!((Rational::new(1, 3).unwrap().to_f64() - 1.0/3.0).abs() < 1e-15);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 13), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(6, 0), 6);
    }
}
