#![deny(missing_docs, unsafe_code)]

//! The `manifest` tool produces an application manifest looking for SIFIS API
//! information contained in a determined binary.

mod app_label;
mod error;
mod sifis_api;

pub use app_label::AppLabel;
pub use sifis_api::{ApiLabel, Functionality, Hazard, HazardsKinds};

use symbolic_common::Name;
use symbolic_demangle::{Demangle, DemangleOptions};

use std::fs::{read, File};
use std::io::Write;
use std::path::Path;

use object::{Object, ObjectSymbol};
use tracing::debug;

use error::{Error, Result};
use sifis_api::SifisApi;

// Permanent part of the link to the GitHub repository containing the file
// associated to the version chosen by a developer
const GITHUB_PERMANENT_LINK: &str =
    "https://raw.githubusercontent.com/sifis-home/manifest/master/data";

/// Produce a manifest file from the hazards associated to the SIFIS-API
/// contained in a binary.
///
/// If no specified, the manifest is not written on a buffer.
#[derive(Debug)]
pub struct ManifestProducer(bool);

impl Default for ManifestProducer {
    fn default() -> Self {
        Self::new()
    }
}

impl ManifestProducer {
    /// Creates a new `ManifestProducer` instance.
    pub fn new() -> Self {
        Self(false)
    }

    /// Enables manifest writing on a buffer.
    pub fn enable_write(mut self) -> Self {
        self.0 = true;
        self
    }

    /// Runs manifest producer.
    pub fn run<P: AsRef<Path>>(
        self,
        binary_path: P,
        library_version: &str,
        manifest_path: Option<P>,
    ) -> Result<AppLabel> {
        // Check if binary path is a file.
        if binary_path.as_ref().is_dir() {
            return Err(Error::FormatPath("Path to binary MUST be a file path"));
        }

        // Check if output path is a file.
        if manifest_path
            .as_ref()
            .map_or(false, |p| p.as_ref().is_dir())
        {
            return Err(Error::FormatPath("Path to manifest MUST be a file path"));
        }

        let response_body = if let Some(stripped_path) = library_version.strip_prefix("file://") {
            std::fs::read_to_string(stripped_path)?
        } else {
            // If it contains a / assume a full path
            let hazards_path = if library_version.contains('/') {
                library_version.to_owned()
            } else {
                format!(
            "{GITHUB_PERMANENT_LINK}/{library_version}/library-api-hazards-{library_version}.json")
            };

            // Download SIFIS-HOME library version and retrieve hazards and APIs
            // information
            reqwest::blocking::get(hazards_path)?.text()?
        };

        let hazards: SifisApi = serde_json::from_str(&response_body)?;

        // Obtain application manifest
        let application_manifest = read_binary(binary_path, hazards)?;

        // Choose whether manifest should be written on some buffer
        if self.0 {
            // Write either on stdout or a file the manifest
            if let Some(manifest_path) = manifest_path {
                let manifest = File::create(manifest_path)?;
                serde_json::to_writer(manifest, &application_manifest)?
            } else {
                let stdout = std::io::stdout();
                let json_data = serde_json::to_string_pretty(&application_manifest)?;

                writeln!(stdout.lock(), "{}", json_data)?
            }
        }

        Ok(application_manifest)
    }
}

const SIFIS_SYMBOL: &str = "sifis_api::service::SifisApiClient";

fn read_binary<P: AsRef<Path>>(binary_path: P, sifis_api: SifisApi) -> Result<AppLabel> {
    let bin_data = read(binary_path)?;
    let file = object::File::parse(&*bin_data)?;

    let mut output_api_labels = Vec::new();

    let data_exp = file.symbols();
    for exp in data_exp {
        if let Ok(name) = exp.name().map(|name| {
            let name = Name::from(name);
            name.try_demangle(DemangleOptions::name_only()).to_string()
        }) {
            if name.starts_with(SIFIS_SYMBOL) && !name.contains("closure") && exp.is_global() {
                debug!("{name} {:?}", exp);
                for api_label in &sifis_api.api_labels {
                    if name.contains(&api_label.api_name) {
                        output_api_labels.push(api_label.clone());
                    }
                }
            }
        }
    }

    Ok(AppLabel::new(
        "app_name",
        "app_description",
        &sifis_api.version,
        output_api_labels,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    const BINARY_PATH: &str = "demo-binary/demo-binary";
    const SIFIS_API: &str = "file://./data/0.1/library-api-hazards-0.1.json";

    #[test]
    fn check_app_label() {
        let application_manifest = ManifestProducer::new()
            .run(BINARY_PATH, SIFIS_API, None)
            .unwrap();

        assert_eq!(application_manifest,
            AppLabel {
                app_name: "app_name".into(),
                app_description: "app_description".into(),
                sifis_version: "0.1".into(),
                api_labels: vec![
                    ApiLabel {
                        api_name: "turn_lamp_on".into(),
                        api_description: "Turns on a lamp.".into(),
                        behavior_label: vec![
                            Functionality {
                                device_type: "lamp".into(),
                                action: "turn_on".into(),
                            }
                        ],
                        security_label: HazardsKinds {
                            safety: vec![
                                Some(Hazard {
                                    name: "FireHazard".into(),
                                    description: "The execution may cause fire.".into(),
                                    risk_score: Some(2)
                                })
                            ],
                            privacy: vec![
                                Some(Hazard {
                                    name: "LogEnergyConsumption".into(),
                                    description: "The execution allows the app to register information about energy consumption.".into(),
                                    risk_score: None
                                })
                            ],
                            financial: vec![
                                Some(Hazard {
                                    name: "ElectricEnergyConsumption".into(),
                                    description: "The execution enables the device to consume further electricity.".into(),
                                    risk_score: Some(5)
                                })
                            ]
                        }
                    }
                ]
            });
    }
}
