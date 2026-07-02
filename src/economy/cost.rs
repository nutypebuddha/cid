#[derive(Debug, Clone)]
pub struct CostTracker {
    pub gate_costs: Vec<(String, f64)>,
    pub total_cost: f64,
}

impl Default for CostTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl CostTracker {
    pub fn new() -> Self {
        CostTracker {
            gate_costs: Vec::new(),
            total_cost: 0.0,
        }
    }

    pub fn record(&mut self, gate_name: &str, cost: f64) {
        self.gate_costs.push((gate_name.to_string(), cost));
        self.total_cost += cost;
    }

    pub fn average_cost(&self) -> f64 {
        if self.gate_costs.is_empty() {
            return 0.0;
        }
        self.total_cost / self.gate_costs.len() as f64
    }

    pub fn min_cost(&self) -> f64 {
        self.gate_costs.iter().map(|(_, c)| *c).fold(f64::INFINITY, f64::min)
    }

    pub fn max_cost(&self) -> f64 {
        self.gate_costs.iter().map(|(_, c)| *c).fold(f64::NEG_INFINITY, f64::max)
    }

    pub fn reset(&mut self) {
        self.gate_costs.clear();
        self.total_cost = 0.0;
    }
}
