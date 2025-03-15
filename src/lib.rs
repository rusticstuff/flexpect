//! # flexpect
//!
//! `#[flexpect::e(...)]` compiles to `#[expect(...)]` for newer versions of Rust and to `#[allow(...)]` when not supported.
//!
//! ## Purpose
//!
//! This crate enables getting the benefits of `#[expect(...)]` while maintaining a minimum supported Rust version that's earlier than 1.81.
//!
//! ## Usage
//! Add `flexpect` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! flexpect = "0.1.0"
//! ```
//!
//! Then use the `#[flexpect::e(...)]` or `#[flexpect::flexpect(...)]` attributes instead of `#[expect(...)]`:
//!
//! ```rust
//! #[flexpect::e(unused_variables)] // instead of #[expect(unused_variables)]
//! fn example() {
//!     let x = 1;
//! }
//! ```
//!
//! ```rust
//! use flexpect::flexpect;
//!
//! #[flexpect(clippy::clone_on_copy)]  // instead of #[expect(clippy::clone_on_copy)]
//! fn clippy_example() {
//!     let _ = 32.clone();
//! }
//! ```
//!
//! ## How it works
//!
//! - On Rust versions before 1.43.0, the attribute is ignored due to compiler bugs.
//! - From Rust 1.43.0 to 1.80, it translates to `#[allow(...)]`.
//! - From Rust 1.81 onwards, it translates to `#[expect(...)]`.
//!
//! The minimum supported Rust version is 1.38.
//!
//! ## Limitations
//!
//! flexpect does not work as inner attributes (`#![flexpect::e(...)]`) nor on statements, expressions or blocks due to compiler limitations.

extern crate proc_macro;

use proc_macro::TokenStream;

#[rustversion::any(before(1.43.0))]
#[proc_macro_attribute]
pub fn e(_attr_args: TokenStream, item: TokenStream) -> TokenStream {
    // using #[allow(...)] does not work before 1.43.0 due to a bug
    item
}

#[rustversion::all(since(1.43.0), before(1.81))]
#[proc_macro_attribute]
pub fn e(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    create_attr("allow", attr_args, item)
}

#[rustversion::since(1.81)]
#[proc_macro_attribute]
pub fn e(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    create_attr("expect", attr_args, item)
}

#[rustversion::any(before(1.43.0))]
#[proc_macro_attribute]
pub fn flexpect(_attr_args: TokenStream, item: TokenStream) -> TokenStream {
    // using #[allow(...)] does not work before 1.43.0 due to a bug
    item
}

#[rustversion::all(since(1.43.0), before(1.81))]
#[proc_macro_attribute]
pub fn flexpect(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    create_attr("allow", attr_args, item)
}

#[rustversion::since(1.81)]
#[proc_macro_attribute]
pub fn flexpect(attr_args: TokenStream, item: TokenStream) -> TokenStream {
    create_attr("expect", attr_args, item)
}

/// Creates `#[allow_or_expect(attr)] item` as a toke stream
#[rustversion::since(1.43.0)]
fn create_attr(name: &str, args: TokenStream, item: TokenStream) -> TokenStream {
    use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenTree};
    use std::iter::once;
    let mut s = TokenStream::new();
    s.extend(once(TokenTree::from(Punct::new('#', Spacing::Alone))));
    let mut attr_inner = TokenStream::new();
    attr_inner.extend(once(TokenTree::from(Ident::new(name, Span::call_site()))));
    attr_inner.extend(once(TokenTree::from(Group::new(
        Delimiter::Parenthesis,
        args,
    ))));
    s.extend(once(TokenTree::from(Group::new(
        Delimiter::Bracket,
        attr_inner,
    ))));
    s.extend(item);
    s
}
