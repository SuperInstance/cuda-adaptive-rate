//! cuda-adaptive-rate: Token-aware adaptive rate limiting for agent workloads.
//!
//! Tracks per-model token consumption, adapts limits based on error rates
//! and latency. Supports backpressure signaling to upstream callers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Rate limit config per model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLimit {
    pub tokens_per_minute: u32,
    pub requests_per_minute: u32,
    pub max_concurrent: u32,
    pub error_retry_after_ms: u64,
}

/// A rate limit decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateDecision {
    pub allowed: bool,
    pub wait_ms: u64,
    pub reason: String,
}

/// Token bucket state for a model
#[derive(Debug)]
struct TokenBucket {
    tokens: f64,
    max_tokens: f64,
    last_refill: Instant,
    requests: u32,
    concurrent: u32,
    recent_errors: u32,
    recent_total: u32,
}

impl TokenBucket {
    fn new(limit: &ModelLimit) -> Self {
        Self {
            tokens: limit.tokens_per_minute as f64,
            max_tokens: limit.tokens_per_minute as f64 * 1.5, // burst allowance
            last_refill: Instant::now(),
            requests: 0,
            concurrent: 0,
            recent_errors: 0,
            recent_total: 0,
        }
    }

    fn refill(&mut self, tpm: u32) {
        let elapsed = self.last_refill.elapsed().as_secs_f64();
        self.tokens = (self.tokens + elapsed * tpm as f64 / 60.0).min(self.max_tokens);
        self.last_refill = Instant::now();
        // Decay error counter (5-minute window)
        self.recent_errors = (self.recent_errors as f64 * 0.98) as u32;
    }
}

/// The adaptive rate limiter
#[derive(Debug)]
pub struct AdaptiveRateLimiter {
    buckets: HashMap<String, TokenBucket>,
    limits: HashMap<String, ModelLimit>,
}

impl AdaptiveRateLimiter {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
            limits: HashMap::new(),
        }
    }

    pub fn add_model(&mut self, name: &str, limit: ModelLimit) {
        self.limits.insert(name.to_string(), limit);
        self.buckets.insert(name.to_string(), TokenBucket::new(&self.limits[name]));
    }

    /// Check if a request is allowed
    pub fn check(&mut self, model: &str, estimated_tokens: u32) -> RateDecision {
        let bucket = match self.buckets.get_mut(model) {
            Some(b) => b,
            None => return RateDecision { allowed: true, wait_ms: 0, reason: "unknown model, allowing".into() },
        };
        let limit = &self.limits[model];
        bucket.refill(limit.tokens_per_minute);

        // Check concurrent
        if bucket.concurrent >= limit.max_concurrent {
            return RateDecision { allowed: false, wait_ms: 1000, reason: format!("concurrent limit: {}/{}", bucket.concurrent, limit.max_concurrent) };
        }

        // Check tokens
        if bucket.tokens < estimated_tokens as f64 {
            let deficit = estimated_tokens as f64 - bucket.tokens;
            let wait = (deficit / (limit.tokens_per_minute as f64 / 60.0) * 1000.0) as u64;
            return RateDecision { allowed: false, wait_ms: wait, reason: format!("token budget: {:.0} available, {} needed", bucket.tokens, estimated_tokens) };
        }

        // Adaptive: if high error rate, back off
        if bucket.recent_total > 10 {
            let error_rate = bucket.recent_errors as f64 / bucket.recent_total as f64;
            if error_rate > 0.3 {
                return RateDecision { allowed: false, wait_ms: limit.error_retry_after_ms, reason: format!("high error rate: {:.0}%", error_rate * 100.0) };
            }
        }

        RateDecision { allowed: true, wait_ms: 0, reason: "ok".into() }
    }

    /// Record a request start
    pub fn acquire(&mut self, model: &str) {
        if let Some(b) = self.buckets.get_mut(model) {
            b.concurrent += 1;
            b.requests += 1;
        }
    }

    /// Record a request end (success or error)
    pub fn release(&mut self, model: &str, tokens_used: u32, is_error: bool) {
        if let Some(b) = self.buckets.get_mut(model) {
            b.tokens -= tokens_used.min(b.tokens as u32) as f64;
            b.concurrent = b.concurrent.saturating_sub(1);
            b.recent_total += 1;
            if is_error { b.recent_errors += 1; }
        }
    }

    /// Get model names
    pub fn models(&self) -> Vec<&str> {
        self.limits.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_limiter() -> AdaptiveRateLimiter {
        let mut l = AdaptiveRateLimiter::new();
        l.add_model("deepseek-chat", ModelLimit {
            tokens_per_minute: 60000,
            requests_per_minute: 60,
            max_concurrent: 5,
            error_retry_after_ms: 5000,
        });
        l.add_model("hermes-405b", ModelLimit {
            tokens_per_minute: 30000,
            requests_per_minute: 10,
            max_concurrent: 2,
            error_retry_after_ms: 30000,
        });
        l
    }

    #[test]
    fn test_allow_normal() {
        let mut l = make_limiter();
        let d = l.check("deepseek-chat", 1000);
        assert!(d.allowed);
    }

    #[test]
    fn test_concurrent_limit() {
        let mut l = make_limiter();
        for _ in 0..5 { l.acquire("hermes-405b"); }
        let d = l.check("hermes-405b", 100);
        assert!(!d.allowed);
        assert!(d.reason.contains("concurrent"));
    }

    #[test]
    fn test_release_restores() {
        let mut l = make_limiter();
        for _ in 0..5 { l.acquire("hermes-405b"); }
        l.release("hermes-405b", 500, false);
        let d = l.check("hermes-405b", 100);
        assert!(d.allowed);
    }

    #[test]
    fn test_token_budget() {
        let mut l = make_limiter();
        l.release("hermes-405b", 30000, false); // drain tokens
        let d = l.check("hermes-405b", 100);
        assert!(!d.allowed);
        assert!(d.reason.contains("token"));
    }

    #[test]
    fn test_adaptive_backoff() {
        let mut l = make_limiter();
        for _ in 0..20 { l.release("hermes-405b", 100, true); }
        let d = l.check("hermes-405b", 100);
        assert!(!d.allowed);
        assert!(d.reason.contains("error rate"));
    }
}
