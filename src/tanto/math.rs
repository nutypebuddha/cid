// Tanto math operations — adapted for CID integration
// Provides: compute_named dispatch, format_f64, and all math ops

use crate::tanto::get_constant;

/// Dispatch named operations with parsed arguments
/// Returns the computed value if the operation is recognized
pub fn compute_named(op: &[u8], args: &[f64]) -> Option<f64> {
    let s = std::str::from_utf8(op).ok()?;
    let n = args.len();
    match s {
        "add" if n == 2 => Some(args[0] + args[1]),
        "sub" if n == 2 => Some(args[0] - args[1]),
        "mul" if n == 2 => Some(args[0] * args[1]),
        "div" if n == 2 && args[1] != 0.0 => Some(args[0] / args[1]),
        "neg" if n == 1 => Some(-args[0]),
        "abs" if n == 1 => Some(args[0].abs()),
        "hypot" if n == 2 => Some((args[0]*args[0] + args[1]*args[1]).sqrt()),
        "sqrt" if n == 1 => {
            if args[0] < 0.0 { None } else { Some(args[0].sqrt()) }
        }
        "sin" if n == 1 => Some(args[0].sin()),
        "cos" if n == 1 => Some(args[0].cos()),
        "tan" if n == 1 => Some(args[0].tan()),
        "asin" if n == 1 => {
            if args[0] < -1.0 || args[0] > 1.0 { None } else { Some(args[0].asin()) }
        }
        "acos" if n == 1 => {
            if args[0] < -1.0 || args[0] > 1.0 { None } else { Some(args[0].acos()) }
        }
        "atan" if n == 1 => Some(args[0].atan()),
        "atan2" if n == 2 => Some(args[1].atan2(args[0])),
        "exp" if n == 1 => Some(args[0].exp()),
        "ln" if n == 1 => {
            if args[0] <= 0.0 { None } else { Some(args[0].ln()) }
        }
        "log10" if n == 1 => {
            if args[0] <= 0.0 { None } else { Some(args[0].log10()) }
        }
        "log2" if n == 1 => {
            if args[0] <= 0.0 { None } else { Some(args[0].log2()) }
        }
        "pow" if n == 2 => Some(args[0].powf(args[1])),
        "round" if n == 1 => Some(args[0].round()),
        "round" if n == 2 => {
            let factor = 10.0_f64.powf(args[1]);
            Some((args[0] * factor).round() / factor)
        }
        "floor" if n == 1 => Some(args[0].floor()),
        "ceil" if n == 1 => Some(args[0].ceil()),
        "min" if n >= 2 => {
            let mut m = args[0];
            for &a in &args[1..] { if a < m { m = a; } }
            Some(m)
        }
        "max" if n >= 2 => {
            let mut m = args[0];
            for &a in &args[1..] { if a > m { m = a; } }
            Some(m)
        }
        "clamp" if n == 3 => Some(args[0].max(args[1]).min(args[2])),
        "sum" if n >= 1 => Some(args.iter().sum()),
        "avg" if n >= 1 => Some(args.iter().sum::<f64>() / n as f64),
        "pct" if n == 2 => Some(args[0] * args[1] / 100.0),
        "pct_change" if n == 2 => {
            if args[0] == 0.0 { None } else { Some((args[1] - args[0]) / args[0] * 100.0) }
        }
        "split" if n == 2 => {
            if args[1] == 0.0 { None } else { Some((args[0] / args[1] * 100.0).round() / 100.0) }
        }
        "deg2rad" if n == 1 => Some(args[0] * std::f64::consts::PI / 180.0),
        "rad2deg" if n == 1 => Some(args[0] * 180.0 / std::f64::consts::PI),
        "dot" if n == 4 => Some(args[0] * args[2] + args[1] * args[3]),
        "norm" if n == 2 => Some((args[0] * args[0] + args[1] * args[1]).sqrt()),
        // Unit conversions
        "mi_to_km" if n == 1 => Some(args[0] * 1.609344),
        "km_to_mi" if n == 1 => Some(args[0] / 1.609344),
        "mi_to_m" if n == 1 => Some(args[0] * 1609.344),
        "m_to_mi" if n == 1 => Some(args[0] / 1609.344),
        "ft_to_m" if n == 1 => Some(args[0] * 0.3048),
        "m_to_ft" if n == 1 => Some(args[0] / 0.3048),
        "in_to_cm" if n == 1 => Some(args[0] * 2.54),
        "cm_to_in" if n == 1 => Some(args[0] / 2.54),
        "mph_to_kmh" if n == 1 => Some(args[0] * 1.609344),
        "kmh_to_mph" if n == 1 => Some(args[0] / 1.609344),
        "mph_to_ms" if n == 1 => Some(args[0] * 0.44704),
        "ms_to_mph" if n == 1 => Some(args[0] / 0.44704),
        "kmh_to_ms" if n == 1 => Some(args[0] / 3.6),
        "ms_to_kmh" if n == 1 => Some(args[0] * 3.6),
        "knot_to_mph" if n == 1 => Some(args[0] * 1.150779),
        "mph_to_knot" if n == 1 => Some(args[0] / 1.150779),
        "f_to_c" if n == 1 => Some((args[0] - 32.0) * 5.0 / 9.0),
        "c_to_f" if n == 1 => Some(args[0] * 9.0 / 5.0 + 32.0),
        "c_to_k" if n == 1 => Some(args[0] + 273.15),
        "k_to_c" if n == 1 => Some(args[0] - 273.15),
        "lb_to_kg" if n == 1 => Some(args[0] * 0.453592),
        "kg_to_lb" if n == 1 => Some(args[0] / 0.453592),
        "kb_to_mb" if n == 1 => Some(args[0] / 1024.0),
        "mb_to_gb" if n == 1 => Some(args[0] / 1024.0),
        "gb_to_mb" if n == 1 => Some(args[0] * 1024.0),
        "mb_to_kb" if n == 1 => Some(args[0] * 1024.0),
        "b_to_mb" if n == 1 => Some(args[0] / 1048576.0),
        "mb_to_b" if n == 1 => Some(args[0] * 1048576.0),
        "hours_to_min" if n == 1 => Some(args[0] * 60.0),
        "min_to_hours" if n == 1 => Some(args[0] / 60.0),
        "days_to_hours" if n == 1 => Some(args[0] * 24.0),
        "sec_to_min" if n == 1 => Some(args[0] / 60.0),
        // Named constants (0-arg)
        "pi" | "e" | "c" | "c_squared" | "R_air" | "g" | "G" | "h" | "hbar"
        | "kB" | "e_charge" | "me" | "mp" | "NA" | "R" | "atm" | "Rearth" | "GMearth" if n == 0 => {
            let op_str = std::str::from_utf8(op).ok()?;
            get_constant(op_str)
        }
        // Physics/math utilities
        "energy" if n == 1 => Some(args[0] * 89875517873681764.0),
        "gas_temp" if n == 2 => {
            if args[1] == 0.0 { None } else { Some(args[0] / (crate::tanto::get_constant("R_air").unwrap_or(287.058_694_911_492_5) * args[1])) }
        }
        "ratio" if n == 3 => {
            if args[2] == 0.0 { None } else { Some(args[0] * args[1] / args[2]) }
        }
        "dilute" if n == 2 => {
            if args[1] == 0.0 { None } else { Some(args[0] / args[1]) }
        }
        "ppm9" if n == 2 => {
            if args[1] == 0.0 { None } else { Some(args[0] / args[1] * 1e6) }
        }
        "ppm602" if n == 2 => {
            if args[1] == 0.0 { None } else { Some(args[0] / args[1] * 6.02214076e23) }
        }
        "factor15" if n == 2 => Some(args[0] * args[1] * 998003.9920159681),
        _ => None,
    }
}

/// Format a floating-point value to a string representation
pub fn format_f64(val: f64) -> String {
    if val.is_nan() {
        return "NaN".to_string();
    }
    if val.is_infinite() {
        if val > 0.0 {
            return "Infinity".to_string();
        } else {
            return "-Infinity".to_string();
        }
    }
    format!("{}", val)
}
