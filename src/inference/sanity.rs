#[derive(Debug, Clone)]
pub struct SanityRange {
    pub name: &'static str,
    pub unit: &'static str,
    pub min: f64,
    pub max: f64,
    pub typical_min: f64,
    pub typical_max: f64,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct SanityResult {
    pub category: String,
    pub value: f64,
    pub unit: String,
    pub in_range: bool,
    pub in_typical: bool,
    pub range: SanityRange,
}

pub struct SanityChecker;

impl SanityChecker {
    pub fn new() -> Self {
        SanityChecker
    }

    pub fn check(&self, value: f64, category: &str) -> Option<SanityResult> {
        let lower = category.to_lowercase();
        
        // First try exact name match
        if let Some(range) = RANGES.iter().find(|r| r.name.to_lowercase() == lower) {
            let in_range = value >= range.min && value <= range.max;
            let in_typical = value >= range.typical_min && value <= range.typical_max;
            return Some(SanityResult {
                category: range.name.to_string(),
                value,
                unit: range.unit.to_string(),
                in_range,
                in_typical,
                range: range.clone(),
            });
        }
        
        // Then try fuzzy match
        let range = RANGES.iter().find(|r| {
            let name_match = lower.contains(&r.name.to_lowercase());
            let unit_match = r.unit.len() >= 2 && lower.contains(&r.unit.to_lowercase());
            let desc_match = r.description.to_lowercase().contains(&lower);
            name_match || unit_match || desc_match
        })?;

        let in_range = value >= range.min && value <= range.max;
        let in_typical = value >= range.typical_min && value <= range.typical_max;

        Some(SanityResult {
            category: range.name.to_string(),
            value,
            unit: range.unit.to_string(),
            in_range,
            in_typical,
            range: range.clone(),
        })
    }

    pub fn check_all(&self, value: f64) -> Vec<SanityResult> {
        RANGES.iter()
            .map(|range| {
                let in_range = value >= range.min && value <= range.max;
                let in_typical = value >= range.typical_min && value <= range.typical_max;
                SanityResult {
                    category: range.name.to_string(),
                    value,
                    unit: range.unit.to_string(),
                    in_range,
                    in_typical,
                    range: range.clone(),
                }
            })
            .collect()
    }

    pub fn score(&self, value: f64, category: &str) -> f64 {
        if let Some(result) = self.check(value, category) {
            if result.in_typical {
                1.0
            } else if result.in_range {
                0.5
            } else {
                0.0
            }
        } else {
            0.5
        }
    }
}

impl Default for SanityChecker {
    fn default() -> Self {
        Self::new()
    }
}

const RANGES: [SanityRange; 20] = [
    SanityRange { name: "speed_mph", unit: "mph", min: 0.0, max: 700.0, typical_min: 1.0, typical_max: 80.0, description: "vehicle speed in miles per hour" },
    SanityRange { name: "speed_ms", unit: "m/s", min: 0.0, max: 343.0, typical_min: 0.5, typical_max: 50.0, description: "speed in meters per second" },
    SanityRange { name: "speed_kmh", unit: "km/h", min: 0.0, max: 1200.0, typical_min: 1.0, typical_max: 130.0, description: "speed in kilometers per hour" },
    SanityRange { name: "temp_c", unit: "C", min: -273.15, max: 6000.0, typical_min: -20.0, typical_max: 45.0, description: "temperature in Celsius" },
    SanityRange { name: "temp_k", unit: "K", min: 0.0, max: 6300.0, typical_min: 253.0, typical_max: 318.0, description: "temperature in Kelvin" },
    SanityRange { name: "height_m", unit: "m", min: 0.0, max: 9000.0, typical_min: 0.5, typical_max: 2.0, description: "height in meters" },
    SanityRange { name: "height_cm", unit: "cm", min: 0.0, max: 900000.0, typical_min: 50.0, typical_max: 200.0, description: "height in centimeters" },
    SanityRange { name: "weight_kg", unit: "kg", min: 0.0, max: 1e6, typical_min: 2.0, typical_max: 200.0, description: "weight in kilograms" },
    SanityRange { name: "weight_lb", unit: "lb", min: 0.0, max: 2.2e6, typical_min: 5.0, typical_max: 450.0, description: "weight in pounds" },
    SanityRange { name: "energy_j", unit: "J", min: 0.0, max: 1e20, typical_min: 1.0, typical_max: 1e9, description: "energy in joules" },
    SanityRange { name: "power_w", unit: "W", min: 0.0, max: 1e12, typical_min: 10.0, typical_max: 10000.0, description: "power in watts" },
    SanityRange { name: "distance_km", unit: "km", min: 0.0, max: 4e8, typical_min: 0.1, typical_max: 10000.0, description: "distance in kilometers" },
    SanityRange { name: "distance_m", unit: "m", min: 0.0, max: 4e11, typical_min: 0.1, typical_max: 100000.0, description: "distance in meters" },
    SanityRange { name: "time_s", unit: "s", min: 0.0, max: 1e18, typical_min: 0.001, typical_max: 86400.0, description: "time in seconds" },
    SanityRange { name: "price_usd", unit: "USD", min: 0.0, max: 1e12, typical_min: 0.5, typical_max: 100000.0, description: "price in US dollars" },
    SanityRange { name: "percent", unit: "%", min: 0.0, max: 100.0, typical_min: 0.0, typical_max: 100.0, description: "percentage value" },
    SanityRange { name: "pressure_pa", unit: "Pa", min: 0.0, max: 1e12, typical_min: 90000.0, typical_max: 110000.0, description: "pressure in Pascals" },
    SanityRange { name: "voltage_v", unit: "V", min: 0.0, max: 1e9, typical_min: 1.0, typical_max: 240.0, description: "voltage in Volts" },
    SanityRange { name: "current_a", unit: "A", min: 0.0, max: 1e9, typical_min: 0.001, typical_max: 100.0, description: "current in Amperes" },
    SanityRange { name: "frequency_hz", unit: "Hz", min: 0.0, max: 1e18, typical_min: 20.0, typical_max: 20000.0, description: "frequency in Hertz" },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_speed() {
        let checker = SanityChecker::new();
        let result = checker.check(60.0, "speed_mph").unwrap();
        assert!(result.in_range);
        assert!(result.in_typical);
    }

    #[test]
    fn test_check_speed_absurd() {
        let checker = SanityChecker::new();
        let result = checker.check(50000.0, "speed_mph").unwrap();
        assert!(!result.in_range);
    }

    #[test]
    fn test_check_temp() {
        let checker = SanityChecker::new();
        let result = checker.check(25.0, "temp_c").unwrap();
        assert!(result.in_typical);
    }

    #[test]
    fn test_check_weight() {
        let checker = SanityChecker::new();
        let result = checker.check(70.0, "weight_kg").unwrap();
        assert!(result.in_typical);
    }

    #[test]
    fn test_check_percent() {
        let checker = SanityChecker::new();
        let result = checker.check(50.0, "percent").unwrap();
        assert!(result.in_range);
    }

    #[test]
    fn test_check_percent_over() {
        let checker = SanityChecker::new();
        let result = checker.check(150.0, "percent").unwrap();
        assert!(!result.in_range);
    }

    #[test]
    fn test_score_in_range() {
        let checker = SanityChecker::new();
        let s = checker.score(25.0, "temp_c");
        assert_eq!(s, 1.0);
    }

    #[test]
    fn test_score_out_of_range() {
        let checker = SanityChecker::new();
        let s = checker.score(50000.0, "speed_mph");
        assert_eq!(s, 0.0);
    }

    #[test]
    fn test_check_all() {
        let checker = SanityChecker::new();
        let results = checker.check_all(25.0);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_unknown_category() {
        let checker = SanityChecker::new();
        assert!(checker.check(1.0, "unknown_category").is_none());
    }
}
