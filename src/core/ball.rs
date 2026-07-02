#[derive(Debug, Clone)]
pub struct TokenCandidate {
    pub id: u32,
    pub token: String,
    pub logit: f64,
    pub probability: f64,
}

impl TokenCandidate {
    pub fn new(id: u32, token: &str, logit: f64) -> Self {
        let probability = 1.0 / (1.0 + (-logit).exp());
        TokenCandidate {
            id,
            token: token.to_string(),
            logit,
            probability,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GateResult {
    pub gate: super::pin::Gate,
    pub passed: bool,
    pub score: f64,
    pub reason: Option<String>,
}

impl GateResult {
    pub fn passed(gate: super::pin::Gate, score: f64) -> Self {
        GateResult {
            gate,
            passed: true,
            score,
            reason: None,
        }
    }

    pub fn failed(gate: super::pin::Gate, score: f64, reason: &str) -> Self {
        GateResult {
            gate,
            passed: false,
            score,
            reason: Some(reason.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ball {
    pub candidate: TokenCandidate,
    pub gate_results: Vec<GateResult>,
    pub total_score: f64,
    pub validated: bool,
}

impl Ball {
    pub fn new(candidate: TokenCandidate) -> Self {
        Ball {
            candidate,
            gate_results: Vec::new(),
            total_score: 0.0,
            validated: false,
        }
    }

    pub fn add_result(&mut self, result: GateResult) {
        self.gate_results.push(result);
        self.recalculate_score();
    }

    pub fn recalculate_score(&mut self) {
        if self.gate_results.is_empty() {
            self.total_score = 0.0;
            self.validated = false;
            return;
        }

        let total: f64 = self.gate_results.iter().map(|r| r.score).sum();
        let count = self.gate_results.len() as f64;
        self.total_score = total / count;
        self.validated = self.gate_results.iter().all(|r| r.passed);
    }

    pub fn passed_gate(&self, gate: super::pin::Gate) -> bool {
        self.gate_results.iter().any(|r| r.gate == gate && r.passed)
    }

    pub fn failed_gate(&self, gate: super::pin::Gate) -> bool {
        self.gate_results.iter().any(|r| r.gate == gate && !r.passed)
    }

    pub fn all_passed(&self) -> bool {
        self.validated
    }
}
