extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs;

use proc_macro2::TokenStream as TokenStream2;
use syn::{
    braced, parenthesized,
    parse::{self, Parse, ParseStream, Parser},
    punctuated::{self, Punctuated},
    token::Brace,
    Expr, Ident, Item, LitBool, LitInt, Path, Token,
};

use quote::{quote, TokenStreamExt};

fn parse(attr: TokenStream2, item: TokenStream2) -> Result<TokenStream2, syn::parse::Error> {
    // println!("attr: {:?}", attr);
    let mut attrs: Attr = syn::parse2(attr)?;
    let module: Module = syn::parse2(item)?;
    let mut next_pass = None;
    // let mut next_passes = None;
    for kv in &mut attrs.attrs {
        match &*kv.0.to_string() {
            "passes" => {
                println!("here we go");
                // println!("r {:?}", e.right);
            }
            _ => {}
        }
        //      if &*id == "passes" {
        //                         match &mut *e.right {
        //                             Expr::Array(a) => {
        //                                 if let Some(e) = a.elems.pop() {
        //                                     let e = e.into_value();
        //                                     match e {
        //                                         Expr::Path(p) => match p.path.get_ident() {
        //                                             Some(i) => {
        //                                                 next_pass = Some(i.clone());
        //                                                 next_passes = Some(a);
        //                                             }
        //                                             _ => {
        //                                                 println!("error identifier");
        //                                             }
        //                                         },
        //                                         _ => {
        //                                             println!("expected identifier")
        //                                         }
        //                                     }
        //                                 } else {
        //                                     println!("error no next pass");
        //                                 }
        //                             }
        //                             _ => {
        //                                 println!("expected []")
        //                             }
        //                         }
        //                     }
        //                 }
        //                 _ => {
        //                     println!("skipping attribute")
        //                 }
        //             },
        //             _ => {
        //                 println!("expected identifier")
        //             }
        //         }
        //     }
        //     _ => {}
        // }
    }

    let next_pass: Ident = next_pass.unwrap();
    // let next_passes : = next_passes.unwrap();
    let items = module.items;

    let ts = quote! {
        // #[ #next_pass(passes = #next_passes)]
        #[passes = [pass2]]
        mod pass1 {

            #(#items)*

        }
    };

    Ok(ts.into())
}

// Attributes are comma separated Expr:s
pub(crate) struct Attr {
    pub attrs: Vec<(Ident, Expr)>,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let pun: Punctuated<syn::ExprAssign, Token![,]> =
            input.parse_terminated(syn::ExprAssign::parse)?;

        println!("here");
        let mut attrs = vec![];
        for ea in pun {
            let l = ea.left;
            let l = quote! {#l};

            let id = syn::parse::<Ident>(l.into())?;
            println!("ident {}, {:?}", id, (*ea.right).clone());
            attrs.push((id, *ea.right));
        }

        Ok(Attr { attrs })
    }
}

// The module is a vector of Item:s
// We don't care about the module identifier
pub(crate) struct Module {
    pub items: Vec<Item>,
}

impl Parse for Module {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        fn parse_items(input: ParseStream<'_>) -> parse::Result<Vec<Item>> {
            let mut items = vec![];

            while !input.is_empty() {
                items.push(input.parse()?);
            }

            Ok(items)
        }

        let content;

        let _mod_token: Token![mod] = input.parse()?;
        let _ident: Ident = input.parse()?;
        let _brace_token: Brace = braced!(content in input);
        let items = content.call(parse_items)?;

        Ok(Module { items })
    }
}

#[proc_macro_attribute]
pub fn app(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("app_start");
    let ts = match parse(attr.into(), item.into()) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    // Try to write the expanded code to disk
    if std::path::Path::new("target").exists() {
        fs::write("target/ts.rs", ts.to_string()).ok();
    }
    println!("app_end");
    ts.into()
}
