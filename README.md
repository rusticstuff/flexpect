# flexpect

`flexpect` is a flexible linting expectation library for Rust that adapts to different compiler versions. It provides procedural macros to handle lint expectations in a version-dependent manner.

## Purpose

The main purpose of this crate is to provide a way to expect or allow certain compiler warnings or lints, adapting its behavior based on the Rust compiler version being used. This is particularly useful for projects that need to support a wide range of Rust versions while maintaining consistent linting behavior.

## Features

- Version-dependent lint handling:
  - Before Rust 1.43.0: No-op (due to a bug in earlier versions)
  - Rust 1.43.0 to 1.80: Uses `#[allow(...)]`
  - Rust 1.81 and later: Uses `#[expect(...)]`
- Supports both compiler warnings and Clippy lints
- Can be applied to individual functions or entire modules

## Usage

Add `flexpect` to your `Cargo.toml`:

```toml
[dependencies]
flexpect = "0.1.0"
```

Then use the `#[flexpect::flexpect(...)]` attribute to expect or allow lints:

```rust
#[flexpect::flexpect(unused_variables)]
fn example() {
    let x = 1; // This unused variable will be allowed or expected, depending on the Rust version
}

#[flexpect::flexpect(clippy::clone_on_copy)]
fn clippy_example() {
    let _ = 32.clone(); // This Clippy lint will be allowed or expected
}
```

## Limitations

## How it works

- On Rust versions before 1.43.0, the attribute does nothing due to a bug in earlier versions.
- From Rust 1.43.0 to 1.80, it translates to `#[allow(...)]`.
- From Rust 1.81 onwards, it translates to `#[expect(...)]`.

This allows your code to compile without unexpected warnings across different Rust versions while still maintaining lint expectations in newer versions.

The minimum supported Rust version is 1.38.0.

## License

[Insert your chosen license here]

## Contributing

[Insert contribution guidelines here]
