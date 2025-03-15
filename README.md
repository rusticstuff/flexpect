# flexpect

`#[flexpect::e(...)]` compiles to `#[expect(...)]` for newer versions of Rust and to `#[allow(...)]` when not supported.

## Purpose

This crate enables getting the benefits of `#[expect(...)]` while maintaining a minimum supported Rust version that's earlier than 1.81.

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

## How it works

- On Rust versions before 1.43.0, the attribute translates to nothing due to a bug in earlier versions.
- From Rust 1.43.0 to 1.80, it translates to `#[allow(...)]`.
- From Rust 1.81 onwards, it translates to `#[expect(...)]`.

The minimum supported Rust version is 1.38.

## Limitations

flexpect does not work as inner attributes (`#![flexpect::e(...)]`) nor on statements, expressions or blocks due to compiler limitations.
