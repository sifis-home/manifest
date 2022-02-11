use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct SifisApi {
    pub(crate) version: String,
    pub(crate) api_labels: Vec<ApiLabel>,
}

/// API Information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiLabel {
    /// API name
    pub api_name: String,
    /// API description
    pub description: String,
    /// API hazards information
    pub security_label: HazardsKinds,
}

/// Hazards kinds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HazardsKinds {
    /// Safety hazards
    pub safety: Vec<Hazard>,
    /// Privacy hazards
    pub privacy: Vec<Hazard>,
    /// Financial hazards
    pub financial: Vec<Hazard>,
}

/// Hazard information,
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hazard {
    /// Name
    pub name: String,
    /// Description
    pub description: String,
    /// Optional risk score
    pub risk_score: Option<f64>,
}
