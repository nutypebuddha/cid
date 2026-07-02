/// Conversation tracker - tracks token usage across LLM conversations.
/// Helps stay within free tier limits.

use std::time::Instant;

#[derive(Debug, Clone)]
pub struct ConversationTracker {
    conversations: Vec<Conversation>,
    daily_stats: DailyStats,
    limits: DailyLimits,
}

#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: String,
    pub started_at: Instant,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub quality_scores: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct DailyStats {
    pub date: String, // YYYY-MM-DD
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_cache_hits: u64,
    pub total_cache_misses: u64,
    pub total_conversations: usize,
    pub avg_quality: f64,
    pub quality_samples: usize,
}

#[derive(Debug, Clone)]
pub struct DailyLimits {
    pub max_input_tokens: u64,
    pub max_output_tokens: u64,
    pub max_conversations: usize,
    pub max_cache_entries: usize,
}

#[derive(Debug, Clone)]
pub struct UsageReport {
    pub input_tokens_used: u64,
    pub input_tokens_remaining: u64,
    pub input_usage_percent: f64,
    pub output_tokens_used: u64,
    pub output_tokens_remaining: u64,
    pub output_usage_percent: f64,
    pub conversations_today: usize,
    pub conversations_remaining: usize,
    pub cache_hit_rate: f64,
    pub avg_quality: f64,
    pub estimated_cost_savings: f64,
}

impl ConversationTracker {
    pub fn new(limits: DailyLimits) -> Self {
        ConversationTracker {
            conversations: Vec::new(),
            daily_stats: DailyStats::today(),
            limits,
        }
    }

    pub fn with_default_limits() -> Self {
        Self::new(DailyLimits::default())
    }

    /// Start tracking a new conversation.
    pub fn start_conversation(&mut self, id: &str) {
        self.conversations.push(Conversation {
            id: id.to_string(),
            started_at: Instant::now(),
            input_tokens: 0,
            output_tokens: 0,
            cache_hits: 0,
            cache_misses: 0,
            quality_scores: Vec::new(),
        });
        self.daily_stats.total_conversations += 1;
    }

    /// Record token usage for a conversation.
    pub fn record_usage(&mut self, conversation_id: &str, input_tokens: u64, output_tokens: u64) {
        if let Some(conv) = self.conversations.iter_mut().find(|c| c.id == conversation_id) {
            conv.input_tokens += input_tokens;
            conv.output_tokens += output_tokens;
        }

        self.daily_stats.total_input_tokens += input_tokens;
        self.daily_stats.total_output_tokens += output_tokens;
    }

    /// Record a cache hit.
    pub fn record_cache_hit(&mut self, conversation_id: &str) {
        if let Some(conv) = self.conversations.iter_mut().find(|c| c.id == conversation_id) {
            conv.cache_hits += 1;
        }
        self.daily_stats.total_cache_hits += 1;
    }

    /// Record a cache miss.
    pub fn record_cache_miss(&mut self, conversation_id: &str) {
        if let Some(conv) = self.conversations.iter_mut().find(|c| c.id == conversation_id) {
            conv.cache_misses += 1;
        }
        self.daily_stats.total_cache_misses += 1;
    }

    /// Record a quality score.
    pub fn record_quality(&mut self, conversation_id: &str, score: f64) {
        if let Some(conv) = self.conversations.iter_mut().find(|c| c.id == conversation_id) {
            conv.quality_scores.push(score);
        }

        // Update daily average
        let total = self.daily_stats.avg_quality * self.daily_stats.quality_samples as f64;
        self.daily_stats.quality_samples += 1;
        self.daily_stats.avg_quality = (total + score) / self.daily_stats.quality_samples as f64;
    }

    /// Get current usage report.
    pub fn report(&self) -> UsageReport {
        let input_remaining = self.limits.max_input_tokens.saturating_sub(self.daily_stats.total_input_tokens);
        let output_remaining = self.limits.max_output_tokens.saturating_sub(self.daily_stats.total_output_tokens);
        let convs_remaining = self.limits.max_conversations.saturating_sub(self.daily_stats.total_conversations);

        let total_cache = self.daily_stats.total_cache_hits + self.daily_stats.total_cache_misses;
        let cache_hit_rate = if total_cache > 0 {
            self.daily_stats.total_cache_hits as f64 / total_cache as f64
        } else {
            0.0
        };

        // Estimate cost savings from cache hits
        // Assume ~$0.002 per 1K tokens saved
        let tokens_saved = self.daily_stats.total_cache_hits * 500; // Assume 500 tokens per cached response
        let estimated_cost_savings = (tokens_saved as f64 / 1000.0) * 0.002;

        UsageReport {
            input_tokens_used: self.daily_stats.total_input_tokens,
            input_tokens_remaining: input_remaining,
            input_usage_percent: if self.limits.max_input_tokens > 0 {
                (self.daily_stats.total_input_tokens as f64 / self.limits.max_input_tokens as f64) * 100.0
            } else {
                0.0
            },
            output_tokens_used: self.daily_stats.total_output_tokens,
            output_tokens_remaining: output_remaining,
            output_usage_percent: if self.limits.max_output_tokens > 0 {
                (self.daily_stats.total_output_tokens as f64 / self.limits.max_output_tokens as f64) * 100.0
            } else {
                0.0
            },
            conversations_today: self.daily_stats.total_conversations,
            conversations_remaining: convs_remaining,
            cache_hit_rate,
            avg_quality: self.daily_stats.avg_quality,
            estimated_cost_savings,
        }
    }

    /// Check if we can afford more usage.
    pub fn can_afford(&self, input_tokens: u64, output_tokens: u64) -> bool {
        self.daily_stats.total_input_tokens + input_tokens <= self.limits.max_input_tokens
            && self.daily_stats.total_output_tokens + output_tokens <= self.limits.max_output_tokens
    }

    /// Check if we should use cache instead of API call.
    pub fn should_cache(&self) -> bool {
        let report = self.report();
        // Use cache if we're over 50% of limits
        report.input_usage_percent > 50.0 || report.output_usage_percent > 50.0
    }

    /// Reset daily stats (call at start of new day).
    pub fn reset_daily(&mut self) {
        self.daily_stats = DailyStats::today();
    }

    /// Get conversation by ID.
    pub fn get_conversation(&self, id: &str) -> Option<&Conversation> {
        self.conversations.iter().find(|c| c.id == id)
    }

    /// Get all active conversations.
    pub fn active_conversations(&self) -> &[Conversation] {
        &self.conversations
    }
}

impl DailyStats {
    fn today() -> Self {
        // Simple date string - in real implementation, use chrono or similar
        DailyStats {
            date: "2026-06-30".to_string(), // Placeholder
            total_input_tokens: 0,
            total_output_tokens: 0,
            total_cache_hits: 0,
            total_cache_misses: 0,
            total_conversations: 0,
            avg_quality: 0.0,
            quality_samples: 0,
        }
    }
}

impl Default for DailyLimits {
    fn default() -> Self {
        DailyLimits {
            max_input_tokens: 1_000_000,   // 1M tokens/day
            max_output_tokens: 500_000,     // 500K tokens/day
            max_conversations: 100,         // 100 conversations/day
            max_cache_entries: 1000,        // 1000 cached responses
        }
    }
}

impl Default for ConversationTracker {
    fn default() -> Self {
        Self::with_default_limits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracker_new() {
        let tracker = ConversationTracker::new(DailyLimits::default());
        assert_eq!(tracker.conversations.len(), 0);
    }

    #[test]
    fn test_start_conversation() {
        let mut tracker = ConversationTracker::new(DailyLimits::default());
        tracker.start_conversation("conv-1");
        assert_eq!(tracker.conversations.len(), 1);
        assert_eq!(tracker.daily_stats.total_conversations, 1);
    }

    #[test]
    fn test_record_usage() {
        let mut tracker = ConversationTracker::new(DailyLimits::default());
        tracker.start_conversation("conv-1");
        tracker.record_usage("conv-1", 100, 200);

        let report = tracker.report();
        assert_eq!(report.input_tokens_used, 100);
        assert_eq!(report.output_tokens_used, 200);
    }

    #[test]
    fn test_record_cache_hit() {
        let mut tracker = ConversationTracker::new(DailyLimits::default());
        tracker.start_conversation("conv-1");
        tracker.record_cache_hit("conv-1");
        tracker.record_cache_hit("conv-1");
        tracker.record_cache_miss("conv-1");

        let report = tracker.report();
        assert_eq!(report.cache_hit_rate, 2.0 / 3.0);
    }

    #[test]
    fn test_record_quality() {
        let mut tracker = ConversationTracker::new(DailyLimits::default());
        tracker.start_conversation("conv-1");
        tracker.record_quality("conv-1", 0.9);
        tracker.record_quality("conv-1", 0.8);

        let report = tracker.report();
        assert!((report.avg_quality - 0.85).abs() < 0.01);
    }

    #[test]
    fn test_can_afford() {
        let tracker = ConversationTracker::new(DailyLimits::default());
        assert!(tracker.can_afford(100, 200));
        assert!(tracker.can_afford(1_000_000, 500_000));
        assert!(!tracker.can_afford(1_000_001, 500_000));
    }

    #[test]
    fn test_should_cache() {
        let mut tracker = ConversationTracker::new(DailyLimits::default());
        assert!(!tracker.should_cache()); // Not at 50% yet

        // Simulate heavy usage
        tracker.daily_stats.total_input_tokens = 600_000; // 60% of 1M
        assert!(tracker.should_cache());
    }

    #[test]
    fn test_report() {
        let mut tracker = ConversationTracker::new(DailyLimits::default());
        tracker.start_conversation("conv-1");
        tracker.record_usage("conv-1", 1000, 2000);
        tracker.record_cache_hit("conv-1");

        let report = tracker.report();
        assert!(report.input_usage_percent > 0.0);
        assert!(report.output_usage_percent > 0.0);
        assert!(report.conversations_today == 1);
    }

    #[test]
    fn test_reset_daily() {
        let mut tracker = ConversationTracker::new(DailyLimits::default());
        tracker.start_conversation("conv-1");
        tracker.record_usage("conv-1", 1000, 2000);

        tracker.reset_daily();
        let report = tracker.report();
        assert_eq!(report.input_tokens_used, 0);
        assert_eq!(report.output_tokens_used, 0);
    }
}
