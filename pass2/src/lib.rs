extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::fs;

use quote::quote;

#[proc_macro_attribute]
pub fn pass2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ts = quote! {
        fn answer() -> u32 { 42 }
        fn main() {
            println!("answer {}", answer());
        }
    };

    // Try to write the expanded code to disk
    if std::path::Path::new("target").exists() {
        fs::write("target/pass2.rs", ts.to_string()).ok();
    }
    ts.into()
}
