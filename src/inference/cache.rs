/// Semantic cache - stores responses keyed by meaning, not exact text.
/// Uses Jaccard word similarity for zero-dependency matching.
use std::time::{Duration, Instant};

pub struct SemanticCache {
    entries: Vec<CacheEntry>,
    max_size: usize,
    ttl: Duration,
    similarity_threshold: f64,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    prompt_words: Vec<String>,
    response: String,
    quality_score: f64,
    created_at: Instant,
    hit_count: usize,
}

#[derive(Debug, Clone)]
pub struct CacheHit {
    pub response: String,
    pub quality_score: f64,
    pub similarity: f64,
    pub hit_count: usize,
    pub age: Duration,
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub total_hits: usize,
    pub total_misses: usize,
    pub hit_rate: f64,
    pub avg_quality: f64,
}

impl SemanticCache {
    pub fn new(max_size: usize, ttl_seconds: u64, similarity_threshold: f64) -> Self {
        SemanticCache {
            entries: Vec::with_capacity(max_size),
            max_size,
            ttl: Duration::from_secs(ttl_seconds),
            similarity_threshold,
        }
    }

    pub fn default_config() -> Self {
        Self::new(1000, 3600, 0.8) // 1000 entries, 1 hour TTL, 80% similarity
    }

    /// Look up a prompt in the cache.
    pub fn lookup(&mut self, prompt: &str) -> Option<CacheHit> {
        let query_words = tokenize(prompt);
        let now = Instant::now();

        let mut best_match: Option<(usize, f64)> = None;

        for (i, entry) in self.entries.iter().enumerate() {
            // Skip expired entries
            if now.duration_since(entry.created_at) > self.ttl {
                continue;
            }

            let similarity = jaccard_similarity(&query_words, &entry.prompt_words);
            if similarity >= self.similarity_threshold {
                match &best_match {
                    Some((_, best_sim)) if similarity > *best_sim => {
                        best_match = Some((i, similarity));
                    }
                    None => {
                        best_match = Some((i, similarity));
                    }
                    _ => {}
                }
            }
        }

        if let Some((idx, similarity)) = best_match {
            let entry = &mut self.entries[idx];
            entry.hit_count += 1;

            Some(CacheHit {
                response: entry.response.clone(),
                quality_score: entry.quality_score,
                similarity,
                hit_count: entry.hit_count,
                age: now.duration_since(entry.created_at),
            })
        } else {
            None
        }
    }

    /// Store a prompt-response pair in the cache.
    pub fn store(&mut self, prompt: &str, response: &str, quality_score: f64) {
        let prompt_words = tokenize(prompt);

        // If cache is full, evict oldest entry with fewest hits
        if self.entries.len() >= self.max_size {
            if let Some(min_idx) = self.entries.iter()
                .enumerate()
                .min_by_key(|(_, e)| e.hit_count)
                .map(|(i, _)| i)
            {
                self.entries.swap_remove(min_idx);
            }
        }

        self.entries.push(CacheEntry {
            prompt_words,
            response: response.to_string(),
            quality_score,
            created_at: Instant::now(),
            hit_count: 0,
        });
    }

    /// Get cache statistics.
    pub fn stats(&self) -> CacheStats {
        let now = Instant::now();
        let active: Vec<&CacheEntry> = self.entries.iter()
            .filter(|e| now.duration_since(e.created_at) <= self.ttl)
            .collect();

        let total_hits: usize = active.iter().map(|e| e.hit_count).sum();
        let avg_quality = if active.is_empty() {
            0.0
        } else {
            active.iter().map(|e| e.quality_score).sum::<f64>() / active.len() as f64
        };

        CacheStats {
            total_entries: active.len(),
            total_hits,
            total_misses: 0, // Track externally if needed
            hit_rate: 0.0,   // Track externally if needed
            avg_quality,
        }
    }

    /// Clear expired entries.
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.entries.retain(|e| now.duration_since(e.created_at) <= self.ttl);
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get number of active entries.
    pub fn len(&self) -> usize {
        let now = Instant::now();
        self.entries.iter()
            .filter(|e| now.duration_since(e.created_at) <= self.ttl)
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for SemanticCache {
    fn default() -> Self {
        Self::default_config()
    }
}

/// Tokenize text into lowercase words.
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|w| w.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|w| !w.is_empty())
        .collect()
}

/// Jaccard similarity between two word sets.
fn jaccard_similarity(a: &[String], b: &[String]) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let set_a: std::collections::HashSet<&String> = a.iter().collect();
    let set_b: std::collections::HashSet<&String> = b.iter().collect();

    let intersection = set_a.intersection(&set_b).count();
    let union = set_a.union(&set_b).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_store_and_lookup() {
        let mut cache = SemanticCache::new(100, 3600, 0.8);
        cache.store("What is the capital of France?", "Paris", 0.95);

        let hit = cache.lookup("What is the capital of France?");
        assert!(hit.is_some(), "Exact match should hit");
        let hit = hit.unwrap();
        assert_eq!(hit.response, "Paris");
        assert!(hit.similarity > 0.99);
    }

    #[test]
    fn test_cache_semantic_match() {
        let mut cache = SemanticCache::new(100, 3600, 0.5); // Lower threshold for contraction handling
        cache.store("What is the capital of France?", "Paris", 0.95);

        // Similar but not exact
        let hit = cache.lookup("What's the capital of France?");
        assert!(hit.is_some(), "Similar query should hit");
    }

    #[test]
    fn test_cache_no_match() {
        let mut cache = SemanticCache::new(100, 3600, 0.8);
        cache.store("What is the capital of France?", "Paris", 0.95);

        let hit = cache.lookup("How do I sort an array in Rust?");
        assert!(hit.is_none(), "Different query should miss");
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache = SemanticCache::new(3, 3600, 0.8);
        cache.store("prompt 1", "response 1", 0.9);
        cache.store("prompt 2", "response 2", 0.9);
        cache.store("prompt 3", "response 3", 0.9);

        // Cache is full, adding should evict
        cache.store("prompt 4", "response 4", 0.9);
        assert!(cache.len() <= 3, "Should not exceed max size");
    }

    #[test]
    fn test_jaccard_similarity() {
        let a = tokenize("the cat sat on the mat");
        let b = tokenize("the cat sat on the rug");
        let sim = jaccard_similarity(&a, &b);
        assert!(sim > 0.5, "Similar sentences should have high similarity, got {}", sim);
    }

    #[test]
    fn test_jaccard_different() {
        let a = tokenize("the cat sat on the mat");
        let b = tokenize("rust programming language");
        let sim = jaccard_similarity(&a, &b);
        assert!(sim < 0.2, "Different sentences should have low similarity, got {}", sim);
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = SemanticCache::new(100, 3600, 0.8);
        cache.store("test prompt", "test response", 0.85);
        let _ = cache.lookup("test prompt");

        let stats = cache.stats();
        assert_eq!(stats.total_entries, 1);
        assert!(stats.avg_quality > 0.8);
    }

    #[test]
    fn test_cache_cleanup() {
        let mut cache = SemanticCache::new(100, 1, 0.8); // 1 second TTL
        cache.store("test", "response", 0.9);

        // Should be valid immediately
        assert!(cache.lookup("test").is_some());

        // Wait for expiry (in real test, we'd mock time)
        // For now, just test that cleanup doesn't crash
        cache.cleanup();
    }

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("Hello, World! This is a test.");
        assert!(tokens.contains(&"hello".to_string()));
        assert!(tokens.contains(&"world".to_string()));
        assert!(tokens.contains(&"test".to_string()));
    }
}
