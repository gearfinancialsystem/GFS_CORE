use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestResult {
    pub eventDate: String,
    pub eventType: String,
    pub payoff: String,
    pub currency: String,
    pub notionalPrincipal: String,
    pub nominalInterestRate: String,
    pub accruedInterest: String,
}
