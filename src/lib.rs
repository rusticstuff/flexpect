extern crate proc_macro;

#[rustversion::any(before(1.43.0), all(nightly, before(2020-02-27)))]
#[proc_macro_attribute]
pub fn expect(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // using #[allow(...)] does not work before 1.43.0 due to a bug
    item
}

#[rustversion::all(
    any(since(1.43.0), all(nightly, since(2020-02-27))),
    any(before(1.81), all(nightly, before(2024-06-27))))]
#[proc_macro_attribute]
pub fn expect(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use proc_macro2::TokenStream;
    use quote::quote;
    let attr = TokenStream::from(attr);
    let item = TokenStream::from(item);
    let expanded = quote! {
        #[allow(#attr)]
        #item
    };
    expanded.into()
}

#[rustversion::any(since(1.81), all(nightly, since(2024-06-27)))]
#[proc_macro_attribute]
pub fn expect(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
    use std::iter::once;
    let mut s = TokenStream::new();
    s.extend(once(TokenTree::from(Punct::new('#', Spacing::Alone))));
    let mut attr_inner = TokenStream::new();
    attr_inner.extend(once(TokenTree::from(Ident::new(
        "allow",
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
