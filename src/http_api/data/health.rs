use serde::{Serialize};

const UNKNOWN: &str = "unknown";
const HEALTHY: &str = "healthy";
const UNHEALTHY: &str = "unhealthy";

#[derive(Serialize)]
pub enum HealthStatus {
    Unknown,
    Unhealthy,
    Healthy,
}

impl Into<&'static str> for HealthStatus {
    fn into(self) -> &'static str {
        match self {
            HealthStatus::Unknown => UNKNOWN,
            HealthStatus::Healthy => HEALTHY,
            HealthStatus::Unhealthy => UNHEALTHY
        }
    }
}

#[derive(Serialize)]
pub struct HealthStatusResponse {
    pub status: HealthStatus
}

impl Default for HealthStatusResponse {
    fn default() -> Self {
        Self{status: HealthStatus::Unknown}
    }
}
