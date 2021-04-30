extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;

#[proc_macro_attribute]
pub fn pass2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ts = quote! {
        fn answer() -> u32 { 42 }
        fn main() {
            println!("answer {}", answer());
        }
    };
    ts.into()
}
