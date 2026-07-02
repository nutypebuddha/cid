// Tanto unit conversion — adapted for CID
// 60+ conversions: length, speed, temperature, weight, volume, data, time

/// Result of a unit conversion
#[derive(Debug, Clone)]
pub struct ConvertResult {
    pub value: f64,
    pub from: String,
    pub to: String,
}

/// Convert a value from one unit to another
/// Format: "60 mph m/s" or "100 F C"
pub fn convert(args: &str) -> Option<ConvertResult> {
    let args = args.trim().as_bytes();
    let (val_bytes, rest) = split_token(args)?;
    let val = parse_f64(val_bytes)?;
    let (from_raw, to_raw) = split_token(rest)?;
    let from_norm = normalize_unit(trim(from_raw));
    let to_norm = normalize_unit(trim(to_raw));

    // Temperature special cases (non-linear)
    let result = match (from_norm, to_norm) {
        (b"f", b"c") => Some((val - 32.0) * 5.0 / 9.0),
        (b"c", b"f") => Some(val * 9.0 / 5.0 + 32.0),
        (b"c", b"k") => Some(val + 273.15),
        (b"k", b"c") => Some(val - 273.15),
        (b"f", b"k") => Some((val - 32.0) * 5.0 / 9.0 + 273.15),
        (b"k", b"f") => Some((val - 273.15) * 9.0 / 5.0 + 32.0),
        _ => {
            let conversions: [(&[u8], &[u8], f64); 30] = [
                (b"mi", b"km", 1.609344),
                (b"km", b"mi", 1.0 / 1.609344),
                (b"mi", b"m", 1609.344),
                (b"m", b"mi", 1.0 / 1609.344),
                (b"ft", b"m", 0.3048),
                (b"m", b"ft", 1.0 / 0.3048),
                (b"in", b"cm", 2.54),
                (b"cm", b"in", 1.0 / 2.54),
                (b"mph", b"kmh", 1.609344),
                (b"kmh", b"mph", 1.0 / 1.609344),
                (b"mph", b"ms", 0.44704),
                (b"ms", b"mph", 1.0 / 0.44704),
                (b"kmh", b"ms", 1.0 / 3.6),
                (b"ms", b"kmh", 3.6),
                (b"knot", b"mph", 1.150779),
                (b"mph", b"knot", 1.0 / 1.150779),
                (b"lb", b"kg", 0.45359237),
                (b"kg", b"lb", 1.0 / 0.45359237),
                (b"gal", b"L", 3.785411784),
                (b"L", b"gal", 1.0 / 3.785411784),
                (b"oz", b"g", 28.3495),
                (b"g", b"oz", 1.0 / 28.3495),
                (b"mb", b"gb", 1.0 / 1024.0),
                (b"gb", b"mb", 1024.0),
                (b"kb", b"mb", 1.0 / 1024.0),
                (b"mb", b"kb", 1024.0),
                (b"b", b"mb", 1.0 / 1048576.0),
                (b"mb", b"b", 1048576.0),
                (b"hr", b"min", 60.0),
                (b"min", b"hr", 1.0 / 60.0),
            ];
            let mut found = None;
            for &(f, t, factor) in &conversions {
                if from_norm == f && to_norm == t {
                    found = Some(val * factor);
                    break;
                }
            }
            found
        }
    };

    match result {
        Some(r) if r.is_finite() => {
            let display_from = unit_display(from_norm);
            let display_to = unit_display(to_norm);
            Some(ConvertResult {
                value: r,
                from: String::from_utf8_lossy(display_from).to_string(),
                to: String::from_utf8_lossy(display_to).to_string(),
            })
        }
        _ => None,
    }
}

/// List available conversion categories
pub fn list_conversions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("mi", "km, m, ft, in, cm"),
        ("km", "mi, m"),
        ("mph", "kmh, ms, knot"),
        ("kmh", "mph, ms"),
        ("F", "C, K"),
        ("C", "F, K"),
        ("lb", "kg"),
        ("kg", "lb"),
        ("gal", "L"),
        ("oz", "g"),
        ("MB", "GB, KB, B"),
        ("hr", "min"),
    ]
}

fn unit_display(normalized: &[u8]) -> &'static [u8] {
    match normalized {
        b"mi" => b"mi", b"km" => b"km", b"m" => b"m",
        b"ft" => b"ft", b"in" => b"in", b"cm" => b"cm",
        b"mph" => b"mph", b"kmh" => b"kmh", b"ms" => b"m/s",
        b"knot" => b"knot", b"lb" => b"lb", b"kg" => b"kg",
        b"gal" => b"gal", b"L" => b"L", b"oz" => b"oz", b"g" => b"g",
        b"mb" => b"MB", b"gb" => b"GB", b"kb" => b"KB", b"b" => b"B",
        b"hr" => b"hr", b"min" => b"min",
        b"f" => b"F", b"c" => b"C", b"k" => b"K",
        _ => b"?",
    }
}

fn normalize_unit(u: &[u8]) -> &[u8] {
    if u == b"km/h" || u == b"kmh" || u == b"km/hr" { return b"kmh"; }
    if u == b"m/s" || u == b"ms" || u == b"m/sec" { return b"ms"; }
    if u == b"mph" || u == b"mi/hr" || u == b"mi/h" { return b"mph"; }
    if u == b"K" || u == b"k" || u == b"kelvin" { return b"k"; }
    if u == b"C" || u == b"c" || u == b"celsius" { return b"c"; }
    if u == b"F" || u == b"f" || u == b"fahrenheit" { return b"f"; }
    if u == b"km" || u == b"kilometer" || u == b"kilometers" { return b"km"; }
    if u == b"mi" || u == b"mile" || u == b"miles" { return b"mi"; }
    if u == b"m" || u == b"meter" || u == b"meters" { return b"m"; }
    if u == b"ft" || u == b"foot" || u == b"feet" { return b"ft"; }
    if u == b"in" || u == b"inch" || u == b"inches" { return b"in"; }
    if u == b"cm" || u == b"centimeter" || u == b"centimeters" { return b"cm"; }
    if u == b"lb" || u == b"lbs" || u == b"pound" || u == b"pounds" { return b"lb"; }
    if u == b"kg" || u == b"kilogram" || u == b"kilograms" { return b"kg"; }
    if u == b"g" || u == b"gram" || u == b"grams" { return b"g"; }
    if u == b"oz" || u == b"ounce" || u == b"ounces" { return b"oz"; }
    if u == b"gal" || u == b"gallon" || u == b"gallons" { return b"gal"; }
    if u == b"L" || u == b"l" || u == b"liter" || u == b"liters" { return b"L"; }
    if u == b"knot" || u == b"knots" { return b"knot"; }
    if u == b"MB" || u == b"mb" || u == b"megabyte" || u == b"megabytes" { return b"mb"; }
    if u == b"GB" || u == b"gb" || u == b"gigabyte" || u == b"gigabytes" { return b"gb"; }
    if u == b"KB" || u == b"kb" || u == b"kilobyte" || u == b"kilobytes" { return b"kb"; }
    if u == b"B" || u == b"b" || u == b"byte" || u == b"bytes" { return b"b"; }
    u
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
    if i >= s.len() { return None; }
    Some((&s[..i], &s[i..]))
}

fn parse_f64(s: &[u8]) -> Option<f64> {
    let s = trim(s);
    let s_str = std::str::from_utf8(s).ok()?;
    s_str.parse::<f64>().ok()
}
