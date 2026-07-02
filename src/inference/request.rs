#[derive(Debug, Clone)]
pub struct ValidationRequest {
    pub text: String,
    pub context: String,
    pub candidates: Vec<String>,
    pub domain: Option<String>,
}

impl ValidationRequest {
    pub fn new(text: &str, context: &str) -> Self {
        ValidationRequest {
            text: text.to_string(),
            context: context.to_string(),
            candidates: Vec::new(),
            domain: None,
        }
    }

    pub fn with_candidates(mut self, candidates: Vec<String>) -> Self {
        self.candidates = candidates;
        self
    }

    pub fn with_domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    pub fn has_candidates(&self) -> bool {
        !self.candidates.is_empty()
    }

    pub fn candidate_count(&self) -> usize {
        self.candidates.len()
    }
}

#[derive(Debug, Clone)]
pub struct ProxyRequest {
    pub prompt: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub validate: bool,
}

impl ProxyRequest {
    pub fn new(prompt: &str, model: &str) -> Self {
        ProxyRequest {
            prompt: prompt.to_string(),
            model: model.to_string(),
            max_tokens: 1000,
            temperature: 0.7,
            validate: true,
        }
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn with_validate(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }
}

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub listen_addr: String,
    pub llm_endpoint: String,
    pub api_key: String,
    pub validate_responses: bool,
    pub max_retries: u32,
    pub timeout_ms: u64,
}

impl ProxyConfig {
    pub fn new(listen_addr: &str, llm_endpoint: &str, api_key: &str) -> Self {
        ProxyConfig {
            listen_addr: listen_addr.to_string(),
            llm_endpoint: llm_endpoint.to_string(),
            api_key: api_key.to_string(),
            validate_responses: true,
            max_retries: 3,
            timeout_ms: 30000,
        }
    }

    pub fn with_validate_responses(mut self, validate: bool) -> Self {
        self.validate_responses = validate;
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
}

#[derive(Debug, Clone)]
pub struct InferenceConfig {
    pub max_candidates: usize,
    pub auto_fix: bool,
    pub strict_mode: bool,
    pub domain: Option<String>,
    pub budget_tokens: u64,
    pub budget_cost_usd: f64,
    pub timeout_ms: u64,
    pub rate_limit_rps: u32,
}

impl InferenceConfig {
    pub fn new() -> Self {
        InferenceConfig {
            max_candidates: 5,
            auto_fix: true,
            strict_mode: false,
            domain: None,
            budget_tokens: 1_000_000,
            budget_cost_usd: 10.0,
            timeout_ms: 5000,
            rate_limit_rps: 100,
        }
    }

    pub fn max_candidates(mut self, max_candidates: usize) -> Self {
        self.max_candidates = max_candidates;
        self
    }

    pub fn auto_fix(mut self, auto_fix: bool) -> Self {
        self.auto_fix = auto_fix;
        self
    }

    pub fn strict_mode(mut self, strict_mode: bool) -> Self {
        self.strict_mode = strict_mode;
        self
    }

    pub fn domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    pub fn budget_tokens(mut self, budget_tokens: u64) -> Self {
        self.budget_tokens = budget_tokens;
        self
    }

    pub fn budget_cost_usd(mut self, budget_cost_usd: f64) -> Self {
        self.budget_cost_usd = budget_cost_usd;
        self
    }

    pub fn timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    pub fn rate_limit_rps(mut self, rate_limit_rps: u32) -> Self {
        self.rate_limit_rps = rate_limit_rps;
        self
    }
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self::new()
    }
}
