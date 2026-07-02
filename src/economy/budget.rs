#[derive(Debug, Clone)]
pub struct Budget {
    pub max_tokens: u64,
    pub used_tokens: u64,
    pub max_cost_usd: f64,
    pub used_cost_usd: f64,
}

impl Budget {
    pub fn new(max_tokens: u64, max_cost_usd: f64) -> Self {
        Budget {
            max_tokens,
            used_tokens: 0,
            max_cost_usd,
            used_cost_usd: 0.0,
        }
    }

    pub fn can_afford_tokens(&self, tokens: u64) -> bool {
        self.used_tokens + tokens <= self.max_tokens
    }

    pub fn can_afford_cost(&self, cost: f64) -> bool {
        self.used_cost_usd + cost <= self.max_cost_usd
    }

    pub fn spend_tokens(&mut self, tokens: u64) -> bool {
        if self.can_afford_tokens(tokens) {
            self.used_tokens += tokens;
            true
        } else {
            false
        }
    }

    pub fn spend_cost(&mut self, cost: f64) -> bool {
        if self.can_afford_cost(cost) {
            self.used_cost_usd += cost;
            true
        } else {
            false
        }
    }

    pub fn remaining_tokens(&self) -> u64 {
        self.max_tokens.saturating_sub(self.used_tokens)
    }

    pub fn remaining_cost(&self) -> f64 {
        (self.max_cost_usd - self.used_cost_usd).max(0.0)
    }

    pub fn token_usage_percent(&self) -> f64 {
        if self.max_tokens == 0 {
            return 0.0;
        }
        (self.used_tokens as f64 / self.max_tokens as f64) * 100.0
    }

    pub fn cost_usage_percent(&self) -> f64 {
        if self.max_cost_usd == 0.0 {
            return 0.0;
        }
        (self.used_cost_usd / self.max_cost_usd) * 100.0
    }

    pub fn is_exhausted(&self) -> bool {
        self.remaining_tokens() == 0 || self.remaining_cost() <= 0.0
    }

    pub fn reset(&mut self) {
        self.used_tokens = 0;
        self.used_cost_usd = 0.0;
    }
}
