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

// Container for comma separated sequences of type T
pub(crate) struct CommaSep<T> {
    pub elems: Vec<T>,
}

impl<T> CommaSep<T> {
    fn remove_first(&mut self) -> Option<T> {
        if self.elems.is_empty() {
            None
        } else {
            let r = self.elems.remove(0);
            Some(r)
        }
    }
}

impl<T> Parse for CommaSep<T>
where
    T: Parse,
{
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let p: Punctuated<T, Token![,]> = input.parse_terminated(T::parse)?;
        let mut elems = vec![];
        for e in p {
            elems.push(e)
        }
        Ok(Self { elems })
    }
}

fn parse(attr: TokenStream2, item: TokenStream2) -> Result<TokenStream2, syn::parse::Error> {
    // println!("attr: {:?}", attr);
    let mut attrs: Attr = syn::parse2(attr)?;
    let module: Module = syn::parse2(item)?;
    let mut next_pass = None;
    let mut next_passes = vec![];
    let mut other_attrs = vec![];
    for (id, e) in &mut attrs.attrs {
        match &*id.to_string() {
            "passes" => {
                println!("passes found");
                println!("e {:?}", e);

                match e {
                    Expr::Array(a) => {
                        let a = a.elems.clone();
                        println!("a---- {:?}", a);
                        let q = quote! {#a};
                        let mut idents = syn::parse::<CommaSep<Ident>>(q.into())?;
                        println!("idents {:?}", idents.elems);
                        next_pass = idents.remove_first();
                        next_passes = idents.elems;
                    }
                    _ => panic!("RTIC ICE"),
                }
            }
            _ => other_attrs.push((id, e)),
        }
    }

    let next_pass = next_pass.unwrap();
    let items = module.items;

    let mut attrs = vec![];
    for (id, expr) in other_attrs {
        attrs.push(quote! {#id = #expr,});
    }

    let ts = quote! {
        #[ #next_pass(passes = [#(#next_passes)*], #(#attrs)*)]
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
            println!("ident {}, {:?}", id, ea.right);
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
        // bad this might panic
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
