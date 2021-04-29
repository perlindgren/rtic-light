extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn pass1(_attr: TokenStream, item: TokenStream) -> TokenStream {
    "
    #[pass2]
    mod h {}
    "
    .parse()
    .unwrap()
}
