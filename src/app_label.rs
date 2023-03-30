use serde::Deserialize;
use serde::Serialize;

use crate::sifis_api::ApiLabel;

/// Sifis application information.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AppLabel {
    /// Application name
    pub app_name: String,
    /// Application description
    pub app_description: String,
    /// Sifis library version
    pub sifis_version: String,
    /// List of hazards associated to API
    pub api_labels: Vec<ApiLabel>,
}

impl AppLabel {
    pub(crate) fn new<S: AsRef<str>>(
        app_name: S,
        app_description: S,
        sifis_version: S,
        api_labels: Vec<ApiLabel>,
    ) -> Self {
        Self {
            app_name: app_name.as_ref().into(),
            app_description: app_description.as_ref().into(),
            sifis_version: sifis_version.as_ref().into(),
            api_labels,
        }
    }
}
