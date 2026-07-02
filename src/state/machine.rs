#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Normal,
    Kakuhen { consecutive_wins: u32 },
    Jitan { fast_mode: bool },
    Koatari { quick_check: bool },
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        State::Normal
    }

    pub fn is_normal(&self) -> bool {
        matches!(self, State::Normal)
    }

    pub fn is_kakuhen(&self) -> bool {
        matches!(self, State::Kakuhen { .. })
    }

    pub fn is_jitan(&self) -> bool {
        matches!(self, State::Jitan { .. })
    }

    pub fn is_koatari(&self) -> bool {
        matches!(self, State::Koatari { .. })
    }

    pub fn consecutive_wins(&self) -> u32 {
        match self {
            State::Kakuhen { consecutive_wins } => *consecutive_wins,
            _ => 0,
        }
    }

    pub fn confidence_multiplier(&self) -> f64 {
        match self {
            State::Normal => 1.0,
            State::Kakuhen { consecutive_wins } => {
                let base = 1.0;
                let boost_per_win = 0.1;
                let max_boost = 2.0;
                let boost = base + (*consecutive_wins as f64 * boost_per_win);
                boost.min(max_boost)
            }
            State::Jitan { fast_mode: true } => 0.8,
            State::Jitan { fast_mode: false } => 1.0,
            State::Koatari { quick_check: true } => 0.6,
            State::Koatari { quick_check: false } => 1.0,
        }
    }

    pub fn validation_depth(&self) -> ValidationDepth {
        match self {
            State::Normal => ValidationDepth::Full,
            State::Kakuhen { .. } => ValidationDepth::Full,
            State::Jitan { fast_mode: true } => ValidationDepth::Minimal,
            State::Jitan { fast_mode: false } => ValidationDepth::Standard,
            State::Koatari { quick_check: true } => ValidationDepth::Quick,
            State::Koatari { quick_check: false } => ValidationDepth::Standard,
        }
    }

    pub fn transition(&mut self, won: bool) {
        match self {
            State::Normal => {
                if won {
                    *self = State::Kakuhen { consecutive_wins: 1 };
                }
            }
            State::Kakuhen { consecutive_wins } => {
                if won {
                    *consecutive_wins += 1;
                    if *consecutive_wins > 5 {
                        *self = State::Jitan { fast_mode: false };
                    }
                } else {
                    *self = State::Jitan { fast_mode: false };
                }
            }
            State::Jitan { fast_mode } => {
                if won {
                    *self = State::Kakuhen { consecutive_wins: 1 };
                } else if !*fast_mode {
                    *self = State::Normal;
                }
            }
            State::Koatari { quick_check: _ } => {
                if won {
                    *self = State::Kakuhen { consecutive_wins: 1 };
                } else {
                    *self = State::Normal;
                }
            }
        }
    }

    pub fn reset(&mut self) {
        *self = State::Normal;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValidationDepth {
    Full,
    Standard,
    Minimal,
    Quick,
}

impl ValidationDepth {
    pub fn description(&self) -> &str {
        match self {
            ValidationDepth::Full => "All gates, full checks",
            ValidationDepth::Standard => "All gates, standard checks",
            ValidationDepth::Minimal => "Core gates only, minimal checks",
            ValidationDepth::Quick => "Quick sanity check only",
        }
    }
}

pub struct StateMachine {
    state: State,
    history: Vec<State>,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            state: State::Normal,
            history: Vec::new(),
        }
    }

    pub fn current(&self) -> &State {
        &self.state
    }

    pub fn transition(&mut self, won: bool) {
        self.history.push(self.state.clone());
        self.state.transition(won);
    }

    pub fn reset(&mut self) {
        self.history.push(self.state.clone());
        self.state.reset();
    }

    pub fn enter_kakuhen(&mut self) {
        self.history.push(self.state.clone());
        self.state = State::Kakuhen { consecutive_wins: 1 };
    }

    pub fn enter_jitan(&mut self, fast_mode: bool) {
        self.history.push(self.state.clone());
        self.state = State::Jitan { fast_mode };
    }

    pub fn enter_koatari(&mut self, quick_check: bool) {
        self.history.push(self.state.clone());
        self.state = State::Koatari { quick_check };
    }

    pub fn history(&self) -> &[State] {
        &self.history
    }

    pub fn confidence_multiplier(&self) -> f64 {
        self.state.confidence_multiplier()
    }

    pub fn validation_depth(&self) -> ValidationDepth {
        self.state.validation_depth()
    }
}
