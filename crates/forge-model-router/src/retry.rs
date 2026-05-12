pub struct RetryPolicy {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub exponential: bool,
}

impl RetryPolicy {
    pub fn backoff_ms(&self, attempt: u32) -> u64 {
        if self.exponential {
            let delay = self.base_delay_ms * 2u64.pow(attempt);
            delay.min(self.max_delay_ms)
        } else {
            self.base_delay_ms
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 8000,
            exponential: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_backoff() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.backoff_ms(0), 1000);
        assert_eq!(policy.backoff_ms(1), 2000);
        assert_eq!(policy.backoff_ms(2), 4000);
        assert_eq!(policy.backoff_ms(3), 8000);
        assert_eq!(policy.backoff_ms(4), 8000); // capped
    }
}
