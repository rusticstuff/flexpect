extern crate proc_macro;

use proc_macro::TokenStream;

#[rustversion::any(before(1.43.0))]
#[proc_macro_attribute]
pub fn expect(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // using #[allow(...)] does not work before 1.43.0 due to a bug
    item
}

#[rustversion::all(since(1.43.0), before(1.81))]
#[proc_macro_attribute]
pub fn expect(attr: TokenStream, item: TokenStream) -> TokenStream {
    replace_attr_with(attr, item, "allow")
}

#[rustversion::since(1.81)]
#[proc_macro_attribute]
pub fn expect(attr: TokenStream, item: TokenStream) -> TokenStream {
    replace_attr_with(attr, item, "expect")
}

#[rustversion::since(1.43.0)]
fn replace_attr_with(attr: TokenStream, item: TokenStream, allow_or_expect: &str) -> TokenStream {
    use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenTree};
    use std::iter::once;
    let mut s = TokenStream::new();
    s.extend(once(TokenTree::from(Punct::new('#', Spacing::Alone))));
    let mut attr_inner = TokenStream::new();
    attr_inner.extend(once(TokenTree::from(Ident::new(
        allow_or_expect,
        Span::call_site(),
    ))));
    attr_inner.extend(once(TokenTree::from(Group::new(
        Delimiter::Parenthesis,
        attr,
    ))));
    s.extend(once(TokenTree::from(Group::new(
        Delimiter::Bracket,
        attr_inner,
    ))));
    s.extend(item);
    s
}
