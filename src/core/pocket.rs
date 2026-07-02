use super::ball::Ball;

#[derive(Debug, Clone)]
pub struct Pocket {
    pub ball: Ball,
    pub kakuhen_triggered: bool,
    pub confidence_boost: f64,
}

impl Pocket {
    pub fn new(ball: Ball) -> Self {
        Pocket {
            ball,
            kakuhen_triggered: false,
            confidence_boost: 1.0,
        }
    }

    pub fn with_kakuhen(mut self, triggered: bool, boost: f64) -> Self {
        self.kakuhen_triggered = triggered;
        self.confidence_boost = boost;
        self
    }

    pub fn should_trigger_kakuhen(&self) -> bool {
        self.ball.all_passed() && self.ball.total_score > 0.8
    }

    pub fn select_best(candidates: Vec<Ball>) -> Option<Pocket> {
        let mut validated: Vec<Ball> = candidates
            .into_iter()
            .filter(|b| b.all_passed())
            .collect();

        if validated.is_empty() {
            return None;
        }

        validated.sort_by(|a, b| {
            b.total_score
                .partial_cmp(&a.total_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let best = validated.into_iter().next()?;
        let kakuhen = best.total_score > 0.9;
        let boost = if kakuhen {
            1.0 + (best.total_score - 0.9) * 10.0
        } else {
            1.0
        };

        Some(Pocket::new(best).with_kakuhen(kakuhen, boost))
    }
}
