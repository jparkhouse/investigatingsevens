# Investigating Sevens

## Running Tests

To run the tests for this project, use the following command:

```sh
cargo test
```

## Generating Test Coverage Report

To generate a test coverage report, you can use `cargo tarpaulin`. First, install `cargo tarpaulin` if you haven't already:

```sh
cargo install cargo-tarpaulin
```

Then, run the following command to generate the test coverage report:

```sh
cargo tarpaulin --out Html
```

The coverage report will be generated in the `tarpaulin-report.html` file.
