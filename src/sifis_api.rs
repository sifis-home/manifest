use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
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
    pub api_description: String,
    /// API functionality label
    pub behavior_label: Vec<Functionality>,
    /// API hazards information
    pub security_label: HazardsKinds,
}

/// Functionality kind.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Functionality {
    /// Kind of device the API involves
    pub device_type: String,
    /// Kind of action over the device
    pub action: String,
}

/// Hazards kinds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HazardsKinds {
    /// Safety hazards
    pub safety: Vec<Option<Hazard>>,
    /// Privacy hazards
    pub privacy: Vec<Option<Hazard>>,
    /// Financial hazards
    pub financial: Vec<Option<Hazard>>,
}

/// Hazard information,
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hazard {
    /// Name
    pub name: String,
    /// Description
    pub description: String,
    /// Optional risk score
    pub risk_score: Option<u8>,
}
