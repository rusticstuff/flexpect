# flexpect

Using `#[flexpect::e(...)]` compiles to `#[expect(...)]` for newer versions of Rust and to `#[allow(...)]` when not supported.

## Purpose

This crate allows you to get the benefits of `#[expect(...)]` while keeping your minimum supported Rust version to before 1.81.

## Details
`#[flexpect::e(...)]` comp
- Version-dependent compilation of `#[flexpect::e(...)]`:
  - Before Rust 1.43.0: No-op (due to a bug in earlier versions)
  - Rust 1.43.0 to 1.80: Uses `#[allow(...)]`
  - Rust 1.81 and later: Uses `#[expect(...)]`

## Usage

Add `flexpect` to your `Cargo.toml`:

```toml
[dependencies]
flexpect = "0.1.0"
```

Then use the `#[flexpect::e(...)]` or `#[flexpect::flexpect(...)]` attributes to expect lints where supported, allow them otherwise:

```rust
#[flexpect::e(unused_variables)]
fn example() {
    let x = 1; // This unused variable will be allowed or expected, depending on the Rust version
}
```

```rust
use flexpect::flexpect;

#[flexpect(clippy::clone_on_copy)]
fn clippy_example() {
    let _ = 32.clone(); // This Clippy lint will be allowed or expected
}
```

## Limitations

flexpect does not work as inner attributes (`#![flexpect::e(...)]`) nor on statements, expressions or blocks due to compiler limitations.

## How it works

- On Rust versions before 1.43.0, the attribute does nothing due to a bug in earlier versions.
- From Rust 1.43.0 to 1.80, it translates to `#[allow(...)]`.
- From Rust 1.81 onwards, it translates to `#[expect(...)]`.

This allows your code to compile without unexpected warnings across different Rust versions while still maintaining lint expectations in newer versions.

The minimum supported Rust version is 1.38.0.
