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

Then use the `#[flexpect::e(...)]` or `#[flexpect::flexpect(...)]` attributes instead of `#[expect(...)]`:

```rust
#[flexpect::e(unused_variables)] // instead of #[expect(unused_variables)]
fn example() {
    let x = 1;
}
```

```rust
use flexpect::flexpect;

#[flexpect(clippy::clone_on_copy)]  // instead of #[expect(clippy::clone_on_copy)]
fn clippy_example() {
    let _ = 32.clone();
}
```

## How it works

- On Rust versions before 1.43.0, the attribute is ignored due to compiler bugs.
- From Rust 1.43.0 to 1.80, it translates to `#[allow(...)]`.
- From Rust 1.81 onwards, it translates to `#[expect(...)]`.

The minimum supported Rust version is 1.38.

## Limitations

flexpect does not work as inner attributes (`#![flexpect::e(...)]`) nor on statements, expressions or blocks due to compiler limitations.
