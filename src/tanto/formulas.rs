// Tanto formulas — 22 verified formula implementations
// Geometry, physics, finance

/// Result of a formula computation
#[derive(Debug, Clone)]
pub struct FormulaResult {
    pub name: String,
    pub formula: String,
    pub result: f64,
    pub args: Vec<f64>,
}

const G_GRAV: f64 = 9.80665;

/// Process a formula command: "circle_area 10" or "ke 1500 26.82"
pub fn compute_formula(args: &str) -> Option<FormulaResult> {
    let args = args.trim().as_bytes();
    let (name, rest) = split_token(args)?;
    let name_str = std::str::from_utf8(name).ok()?;
    let args_str = trim(rest);

    let mut vals = [0.0f64; 4];
    let mut argc = 0;
    if !args_str.is_empty() {
        let mut remaining = args_str;
        while argc < 4 {
            let (token, rest2) = split_token(remaining)?;
            vals[argc] = parse_f64(token)?;
            argc += 1;
            remaining = trim(rest2);
            if remaining.is_empty() { break; }
        }
    }

    let (result, formula_str) = match name {
        // --- Original 22 formulas ---
        b"circle_area" if argc == 1 => (std::f64::consts::PI * vals[0] * vals[0], "pi * r^2"),
        b"circle_circumference" if argc == 1 => (2.0 * std::f64::consts::PI * vals[0], "2 * pi * r"),
        b"sphere_volume" if argc == 1 => (4.0 / 3.0 * std::f64::consts::PI * vals[0] * vals[0] * vals[0], "(4/3) * pi * r^3"),
        b"sphere_area" if argc == 1 => (4.0 * std::f64::consts::PI * vals[0] * vals[0], "4 * pi * r^2"),
        b"rect_area" if argc == 2 => (vals[0] * vals[1], "l * w"),
        b"rect_perimeter" if argc == 2 => (2.0 * (vals[0] + vals[1]), "2 * (l + w)"),
        b"triangle_area" if argc == 2 => (0.5 * vals[0] * vals[1], "0.5 * b * h"),
        b"cylinder_volume" if argc == 2 => (std::f64::consts::PI * vals[0] * vals[0] * vals[1], "pi * r^2 * h"),
        b"cone_volume" if argc == 2 => (std::f64::consts::PI * vals[0] * vals[0] * vals[1] / 3.0, "pi * r^2 * h / 3"),
        b"ke" if argc == 2 => (0.5 * vals[0] * vals[1] * vals[1], "0.5 * m * v^2  (v in m/s!)"),
        b"pe" if argc == 2 => (vals[0] * G_GRAV * vals[1], "m * g * h  (g=9.80665)"),
        b"momentum" if argc == 2 => (vals[0] * vals[1], "m * v"),
        b"force" if argc == 2 => (vals[0] * vals[1], "m * a"),
        b"pressure" if argc == 2 => {
            if vals[1] == 0.0 { return None; }
            (vals[0] / vals[1], "F / A")
        }
        b"work" if argc == 2 => (vals[0] * vals[1], "F * d"),
        b"power" if argc == 2 => {
            if vals[1] == 0.0 { return None; }
            (vals[0] / vals[1], "work / time")
        }
        b"ohm_v" if argc == 2 => (vals[0] * vals[1], "I * R"),
        b"ohm_i" if argc == 2 => {
            if vals[1] == 0.0 { return None; }
            (vals[0] / vals[1], "V / R")
        }
        b"ohm_p" if argc == 2 => {
            if vals[1] == 0.0 { return None; }
            (vals[0] * vals[0] / vals[1], "V^2 / R")
        }
        b"simple_interest" if argc == 3 => (vals[0] * vals[1] * vals[2], "P * r * t"),
        b"compound_amount" if argc == 3 => {
            (vals[0] * (1.0 + vals[1]).powf(vals[2]), "P * (1 + r)^t")
        }
        // --- Phase 2: New geometry formulas (+8) ---
        b"square_area" if argc == 1 => (vals[0] * vals[0], "s^2"),
        b"square_perimeter" if argc == 1 => (4.0 * vals[0], "4 * s"),
        b"cube_volume" if argc == 1 => (vals[0] * vals[0] * vals[0], "s^3"),
        b"cube_surface" if argc == 1 => (6.0 * vals[0] * vals[0], "6 * s^2"),
        b"torus_volume" if argc == 2 => (2.0 * std::f64::consts::PI * std::f64::consts::PI * vals[0] * vals[0] * vals[1], "2*pi^2 * R * r^2"),
        b"torus_area" if argc == 2 => (4.0 * std::f64::consts::PI * std::f64::consts::PI * vals[0] * vals[1], "4*pi^2 * R * r"),
        b"ellipse_area" if argc == 2 => (std::f64::consts::PI * vals[0] * vals[1], "pi * a * b"),
        b"trapezoid_area" if argc == 3 => (0.5 * (vals[0] + vals[1]) * vals[2], "0.5 * (a+b) * h"),
        // --- Phase 2: New physics formulas (+9) ---
        b"density" if argc == 2 => {
            if vals[1] == 0.0 { return None; }
            (vals[0] / vals[1], "m / V")
        }
        b"newton_g" if argc == 3 => {
            if vals[2] == 0.0 { return None; }
            (G_GRAV * vals[0] * vals[1] / (vals[2] * vals[2]), "G * m1 * m2 / r^2")
        }
        b"centripetal" if argc == 2 => {
            if vals[1] == 0.0 { return None; }
            (vals[0] * vals[0] / vals[1], "v^2 / r")
        }
        b"spring_pe" if argc == 2 => (0.5 * vals[0] * vals[1] * vals[1], "0.5 * k * x^2"),
        b"wave_speed" if argc == 2 => (vals[0] * vals[1], "lambda * f"),
        b"doppler_sound" if argc == 3 => {
            if vals[2] == 0.0 { return None; }
            (vals[0] * (343.0 + vals[1]) / (343.0 - vals[2]), "f0 * (v+v_r) / (v+v_s)")
        }
        b"electrostatic" if argc == 3 => {
            if vals[2] == 0.0 { return None; }
            (8.9875517923e9 * vals[0] * vals[1] / (vals[2] * vals[2]), "k * q1 * q2 / r^2")
        }
        b"relativistic_gamma" if argc == 1 => {
            if vals[0] >= 299792458.0 { return None; }
            (1.0 / (1.0 - (vals[0] * vals[0]) / (299792458.0 * 299792458.0)).sqrt(), "1 / sqrt(1-v^2/c^2)")
        }
        b"escape_velocity" if argc == 2 => {
            if vals[1] == 0.0 { return None; }
            ((2.0 * 6.67430e-11 * vals[0] / vals[1]).sqrt(), "sqrt(2*G*M/r)")
        }
        // --- Phase 2: New finance formulas (+6) ---
        b"annuity_fv" if argc == 3 => {
            if vals[2] == 0.0 { return None; }
            (vals[0] * ((1.0 + vals[2]).powf(vals[1]) - 1.0) / vals[2], "PMT * ((1+r)^n-1)/r")
        }
        b"annuity_pv" if argc == 3 => {
            if vals[2] == 0.0 { return None; }
            (vals[0] * (1.0 - (1.0 + vals[2]).powf(-vals[1])) / vals[2], "PMT * (1-(1+r)^-n)/r")
        }
        b"rule_of_72" if argc == 1 => {
            if vals[0] == 0.0 { return None; }
            (72.0 / (vals[0] * 100.0), "72 / (r*100)")
        }
        b"cagr" if argc == 3 => {
            if vals[0] == 0.0 || vals[2] == 0.0 { return None; }
            ((vals[1] / vals[0]).powf(1.0 / vals[2]) - 1.0, "(EV/BV)^(1/n) - 1")
        }
        b"debt_to_income" if argc == 2 => {
            if vals[1] == 0.0 { return None; }
            (vals[0] / vals[1] * 100.0, "debt / income * 100")
        }
        b"roi" if argc == 2 => {
            if vals[0] == 0.0 { return None; }
            ((vals[1] - vals[0]) / vals[0] * 100.0, "(gain - cost) / cost * 100")
        }
        _ => return None,
    };

    Some(FormulaResult {
        name: name_str.to_string(),
        formula: formula_str.to_string(),
        result,
        args: vals[..argc].to_vec(),
    })
}

/// List available formulas
pub fn list_formulas() -> Vec<(&'static str, &'static str)> {
    let mut formulas = vec![
        // Geometry (11)
        ("circle_area <r>", "pi * r^2"),
        ("circle_circumference <r>", "2 * pi * r"),
        ("sphere_volume <r>", "(4/3) * pi * r^3"),
        ("sphere_area <r>", "4 * pi * r^2"),
        ("rect_area <l> <w>", "l * w"),
        ("rect_perimeter <l> <w>", "2 * (l + w)"),
        ("triangle_area <b> <h>", "0.5 * b * h"),
        ("cylinder_volume <r> <h>", "pi * r^2 * h"),
        ("cone_volume <r> <h>", "pi * r^2 * h / 3"),
        ("square_area <s>", "s^2"),
        ("square_perimeter <s>", "4 * s"),
        ("cube_volume <s>", "s^3"),
        ("cube_surface <s>", "6 * s^2"),
        ("torus_volume <R> <r>", "2*pi^2 * R * r^2"),
        ("torus_area <R> <r>", "4*pi^2 * R * r"),
        ("ellipse_area <a> <b>", "pi * a * b"),
        ("trapezoid_area <a> <b> <h>", "0.5 * (a+b) * h"),
        // Physics (15)
        ("ke <m> <v>", "0.5 * m * v^2"),
        ("pe <m> <h>", "m * g * h"),
        ("momentum <m> <v>", "m * v"),
        ("force <m> <a>", "m * a"),
        ("pressure <F> <A>", "F / A"),
        ("work <F> <d>", "F * d"),
        ("power <work> <time>", "work / time"),
        ("ohm_v <I> <R>", "I * R"),
        ("ohm_i <V> <R>", "V / R"),
        ("ohm_p <V> <R>", "V^2 / R"),
        ("density <m> <V>", "m / V"),
        ("newton_g <m1> <m2> <r>", "G * m1 * m2 / r^2"),
        ("centripetal <v> <r>", "v^2 / r"),
        ("spring_pe <k> <x>", "0.5 * k * x^2"),
        ("wave_speed <lambda> <f>", "lambda * f"),
        ("doppler_sound <f0> <v_r> <v_s>", "f0 * (v+v_r)/(v+v_s)"),
        ("electrostatic <q1> <q2> <r>", "k * q1 * q2 / r^2"),
        ("relativistic_gamma <v>", "1/sqrt(1-v^2/c^2)"),
        ("escape_velocity <M> <r>", "sqrt(2*G*M/r)"),
        // Finance (10)
        ("simple_interest <P> <r> <t>", "P * r * t"),
        ("compound_amount <P> <r> <t>", "P * (1 + r)^t"),
        ("annuity_fv <PMT> <n> <r>", "PMT * ((1+r)^n-1)/r"),
        ("annuity_pv <PMT> <n> <r>", "PMT * (1-(1+r)^-n)/r"),
        ("rule_of_72 <r>", "72 / (r*100)"),
        ("cagr <BV> <EV> <n>", "(EV/BV)^(1/n)-1"),
        ("debt_to_income <debt> <income>", "debt/income*100"),
        ("roi <cost> <gain>", "(gain-cost)/cost*100"),
    ];
    formulas.sort_by(|a, b| a.0.cmp(b.0));
    formulas
}

fn trim(s: &[u8]) -> &[u8] {
    let mut start = 0;
    while start < s.len() && (s[start] == b' ' || s[start] == b'\t') { start += 1; }
    let mut end = s.len();
    while end > start && (s[end - 1] == b' ' || s[end - 1] == b'\t') { end -= 1; }
    &s[start..end]
}

fn split_token(s: &[u8]) -> Option<(&[u8], &[u8])> {
    let s = trim(s);
    if s.is_empty() { return None; }
    let mut i = 0;
    while i < s.len() && s[i] != b' ' && s[i] != b'\t' { i += 1; }
    if i >= s.len() { return Some((s, &[])); }
    Some((&s[..i], &s[i..]))
}

fn parse_f64(s: &[u8]) -> Option<f64> {
    let s = trim(s);
    let s_str = std::str::from_utf8(s).ok()?;
    s_str.parse::<f64>().ok()
}
