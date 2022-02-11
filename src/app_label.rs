use serde::Serialize;

use crate::sifis_api::ApiLabel;

/// Sifis application information.
#[derive(Debug, Serialize, PartialEq)]
pub struct AppLabel {
    /// Application name
    pub name: String,
    /// Application description
    pub description: String,
    /// Sifis library version
    pub sifis_version: String,
    /// List of hazards associated to API
    pub api_hazards: Vec<ApiLabel>,
}

impl AppLabel {
    pub(crate) fn new<S: AsRef<str>>(
        name: S,
        description: S,
        sifis_version: S,
        api_hazards: Vec<ApiLabel>,
    ) -> Self {
        Self {
            name: name.as_ref().into(),
            description: description.as_ref().into(),
            sifis_version: sifis_version.as_ref().into(),
            api_hazards,
        }
    }
}
