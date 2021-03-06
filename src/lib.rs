#![deny(missing_docs, unsafe_code)]

//! The `manifest` tool produces an application manifest looking for SIFIS API
//! information contained in a determined binary.

mod app_label;
mod error;
mod sifis_api;

pub use app_label::AppLabel;
pub use sifis_api::{ApiLabel, Hazard, HazardsKinds};

use std::fs::{read, File};
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

use object::{Object, ObjectSymbol};

use error::{Error, Result};
use sifis_api::SifisApi;

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
        sifis_api_path: P,
        manifest_path: Option<P>,
    ) -> Result<AppLabel> {
        // Check if binary path is a file.
        if binary_path.as_ref().is_dir() {
            return Err(Error::FormatPath("Path to binary MUST be a file path"));
        }

        // Check if library label path is a file.
        if sifis_api_path.as_ref().is_dir() {
            return Err(Error::FormatPath(
                "Path to SIFIS API information MUST be a file path",
            ));
        }

        // Check if output path is a file.
        if manifest_path
            .as_ref()
            .map_or(false, |p| p.as_ref().is_dir())
        {
            return Err(Error::FormatPath("Path to manifest MUST be a file path"));
        }

        // Read SIFIS-HOME library version and APIs information
        let file = File::open(sifis_api_path)?;
        let reader = BufReader::new(file);
        let hazards: SifisApi = serde_json::from_reader(reader)?;

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

const SIFIS_SYMBOL: &str = "_ZN9sifis_api";

fn read_binary<P: AsRef<Path>>(binary_path: P, sifis_api: SifisApi) -> Result<AppLabel> {
    let bin_data = read(binary_path)?;
    let file = object::File::parse(&*bin_data)?;

    let mut output_api_labels = Vec::new();

    let data_exp = file.symbols();
    for exp in data_exp {
        if let Ok(name) = exp.name() {
            if name.starts_with(SIFIS_SYMBOL) && !name.contains("closure") {
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
    const SIFIS_API_PATH: &str = "data/library-label.json";

    #[test]
    fn check_app_label() {
        let application_manifest = ManifestProducer::new()
            .run(BINARY_PATH, SIFIS_API_PATH, None)
            .unwrap();

        assert_eq!(application_manifest,
            AppLabel {
                name: "app_name".into(),
                description: "app_description".into(),
                sifis_version: "0.1".into(),
                api_hazards: vec![
                    ApiLabel {
                        api_name: "turn_light_on".into(),
                        description: "Turns on a lamp.".into(),
                        security_label: HazardsKinds {
                            safety: vec![
                                Hazard {
                                    name: "FIRE_HAZARD".into(),
                                    description: "The execution may cause fire.".into(),
                                    risk_score: None
                                }
                            ],
                            privacy: vec![
                                Hazard {
                                    name: "LOG_ENERGY_CONSUMPTION".into(),
                                    description: "The execution allows the app to register information about energy consumption.".into(),
                                    risk_score: None
                                }
                            ],
                            financial: vec![
                                Hazard {
                                    name: "ELECTRIC_ENERGY_CONSUMPTION".into(),
                                    description: "The execution enables the device to consume further electricity.".into(),
                                    risk_score: Some(0.8)
                                }
                            ]
                        }
                    }
                ]
            });
    }
}
