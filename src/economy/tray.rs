#[derive(Debug, Clone)]
pub struct BallEconomy {
    pub tray: u64,
    pub spent: u64,
    pub won: u64,
    pub cost_per_gate: f64,
    pub total_cost: f64,
}

impl BallEconomy {
    pub fn new(initial_tray: u64) -> Self {
        BallEconomy {
            tray: initial_tray,
            spent: 0,
            won: 0,
            cost_per_gate: 0.000001,
            total_cost: 0.0,
        }
    }

    pub fn with_cost(mut self, cost_per_gate: f64) -> Self {
        self.cost_per_gate = cost_per_gate;
        self
    }

    pub fn can_afford(&self, num_gates: usize) -> bool {
        let cost = num_gates as u64;
        self.tray >= cost
    }

    pub fn spend(&mut self, num_gates: usize) -> bool {
        let cost = num_gates as u64;
        if self.tray >= cost {
            self.tray -= cost;
            self.spent += cost;
            self.total_cost += cost as f64 * self.cost_per_gate;
            true
        } else {
            false
        }
    }

    pub fn win(&mut self, amount: u64) {
        self.tray += amount;
        self.won += amount;
    }

    pub fn refund(&mut self, amount: u64) {
        self.tray += amount;
        self.spent -= amount;
    }

    pub fn balance(&self) -> u64 {
        self.tray
    }

    pub fn total_spent(&self) -> u64 {
        self.spent
    }

    pub fn total_won(&self) -> u64 {
        self.won
    }

    pub fn total_cost_usd(&self) -> f64 {
        self.total_cost
    }

    pub fn roi(&self) -> f64 {
        if self.spent == 0 {
            return 0.0;
        }
        (self.won as f64 / self.spent as f64) * 100.0
    }

    pub fn is_bust(&self) -> bool {
        self.tray == 0
    }

    pub fn reset(&mut self, initial: u64) {
        self.tray = initial;
        self.spent = 0;
        self.won = 0;
        self.total_cost = 0.0;
    }
}
