extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn pass2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
