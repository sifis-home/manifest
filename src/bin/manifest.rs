use std::path::PathBuf;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use manifest::ManifestProducer;

#[derive(Parser, Debug)]
struct Opts {
    /// Path to the binary to be analyzed
    #[clap(short, value_parser)]
    binary_path: PathBuf,
    /// Path to the SIFIS-Home library API labels
    #[clap(short, value_parser)]
    library_api_labels_path: PathBuf,
    /// Output path of the produced manifest
    #[clap(short, value_parser)]
    output_path: Option<PathBuf>,
    /// Enable additional information about the underlying process
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let opts = Opts::parse();

    // Enable filter to log the information contained in the lib.
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| {
            if opts.verbose {
                EnvFilter::try_new("debug")
            } else {
                EnvFilter::try_new("info")
            }
        })
        .unwrap();

    // Run tracer.
    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(filter_layer)
        .with_writer(std::io::stderr)
        .init();

    ManifestProducer::new()
        .enable_write()
        .run(
            opts.binary_path,
            opts.library_api_labels_path,
            opts.output_path,
        )
        .unwrap();
}
