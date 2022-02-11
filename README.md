# Manifest Producer

This tool produces an application manifest file containing
SIFIS-Home API information extracted from a determined binary.

A *manifest* is a JSON file consisting of application information and
API hazards.

## Build

To build both `lib` and `manifest` code:

```
cargo build
```

## Run tests

Verify whether all tests pass with the command:

```
cargo test
```

## Produce docs

Generate the final documentation with the command:

```
cargo doc --open --no-deps
```

Remove the `--no-deps` option to build documentation for each dependency.

## View options

To view the list of `manifest` options, run:

```
cargo run -- --help
```

## Usage

To write a *manifest* on `stdout` run:

```
cargo run -- -b /path/to/your/binary -s /path/to/your/sifis-home/library/api/labels/file
```

To write a *manifest* on a file run:

```
cargo run -- -b /path/to/your/binary -s /path/to/your/sifis-home/library/api/labels/file -o /path/to/your/manifest
```

## License

Released under the [MIT License](LICENSE).

## Acknowledgements

This software has been developed in the scope of the H2020 project SIFIS-Home with GA n. 952652.

