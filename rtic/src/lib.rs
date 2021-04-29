extern crate proc_macro;
use proc_macro::TokenStream;
use std::{fs, path::Path};

use proc_macro2::TokenStream as TokenStream2;
// use syn::{
//     // braced, parenthesized,
//     // parse::{self, Parse, ParseStream, Parser},
//     // token::Brace,
//     // Ident, Item, LitBool, LitInt, Token,

// };
// use quote::TokenStreamExt;

fn parse(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    let mut ts: TokenStream2 = "#[pass1]".parse().unwrap();
    ts.extend(item);
    ts.into()
}

#[proc_macro_attribute]
pub fn app(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("app_start");
    let ts = parse(attr.into(), item.into());

    // Try to write the expanded code to disk
    if Path::new("target").exists() {
        fs::write("target/ts.rs", ts.to_string()).ok();
    }
    println!("app_end");
    ts.into()
}
