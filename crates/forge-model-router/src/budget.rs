pub struct BudgetTracker {
    total_tokens: u64,
    total_cost_usd: f64,
    budget_limit_usd: Option<f64>,
}

impl BudgetTracker {
    pub fn new() -> Self {
        Self {
            total_tokens: 0,
            total_cost_usd: 0.0,
            budget_limit_usd: None,
        }
    }

    pub fn set_budget_limit(&mut self, limit_usd: f64) {
        self.budget_limit_usd = Some(limit_usd);
    }

    pub fn record_usage(&mut self, tokens: u32, cost_usd: f64) {
        self.total_tokens += tokens as u64;
        self.total_cost_usd += cost_usd;
    }

    pub fn total_tokens(&self) -> u64 {
        self.total_tokens
    }

    pub fn total_cost(&self) -> f64 {
        self.total_cost_usd
    }

    pub fn is_over_budget(&self) -> bool {
        self.budget_limit_usd
            .map(|limit| self.total_cost_usd > limit)
            .unwrap_or(false)
    }

    pub fn remaining_budget(&self) -> Option<f64> {
        self.budget_limit_usd
            .map(|limit| (limit - self.total_cost_usd).max(0.0))
    }
}

impl Default for BudgetTracker {
    fn default() -> Self {
        Self::new()
    }
}
