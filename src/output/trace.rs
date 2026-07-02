use crate::core::ball::Ball;
use crate::core::pocket::Pocket;
use crate::state::machine::State;

#[derive(Debug, Clone)]
pub struct ValidationTrace {
    pub step: usize,
    pub input: String,
    pub candidates: Vec<Ball>,
    pub selected: Option<Pocket>,
    pub state: State,
    pub timestamp: u64,
}

impl ValidationTrace {
    pub fn new(step: usize, input: &str) -> Self {
        ValidationTrace {
            step,
            input: input.to_string(),
            candidates: Vec::new(),
            selected: None,
            state: State::Normal,
            timestamp: 0,
        }
    }

    pub fn add_candidate(&mut self, ball: Ball) {
        self.candidates.push(ball);
    }

    pub fn set_selected(&mut self, pocket: Pocket) {
        self.selected = Some(pocket);
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn set_timestamp(&mut self, ts: u64) {
        self.timestamp = ts;
    }

    pub fn summary(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Step {}: \"{}\"\n", self.step, self.input));
        output.push_str(&format!("  Candidates: {}\n", self.candidates.len()));
        
        let validated = self.candidates.iter().filter(|b| b.validated).count();
        output.push_str(&format!("  Validated: {}\n", validated));
        
        if let Some(ref pocket) = self.selected {
            output.push_str(&format!("  Selected: \"{}\" (score: {:.4})\n", 
                pocket.ball.candidate.token, pocket.ball.total_score));
        }
        
        output.push_str(&format!("  State: {:?}\n", self.state));
        output
    }
}

pub struct TraceLog {
    pub traces: Vec<ValidationTrace>,
    pub max_traces: usize,
}

impl TraceLog {
    pub fn new(max_traces: usize) -> Self {
        TraceLog {
            traces: Vec::new(),
            max_traces,
        }
    }

    pub fn add(&mut self, trace: ValidationTrace) {
        if self.traces.len() >= self.max_traces {
            self.traces.remove(0);
        }
        self.traces.push(trace);
    }

    pub fn last(&self) -> Option<&ValidationTrace> {
        self.traces.last()
    }

    pub fn get(&self, index: usize) -> Option<&ValidationTrace> {
        self.traces.get(index)
    }

    pub fn len(&self) -> usize {
        self.traces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.traces.is_empty()
    }

    pub fn summary(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("=== TRACE LOG ({} entries) ===\n", self.traces.len()));
        
        for trace in &self.traces {
            output.push_str(&trace.summary());
            output.push('\n');
        }
        
        output
    }

    pub fn reset(&mut self) {
        self.traces.clear();
    }
}
