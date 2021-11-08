/// Main wrapper for RTIC
///
///
extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs;

use proc_macro2::TokenStream as TokenStream2;
use syn::{
    braced, /* parenthesized, */
    parse::{self, Parse, ParseStream, Parser},
    punctuated::Punctuated,
    token::Brace,
    Expr, Ident, Item, /*  LitBool, LitInt, Path*/ Token,
};

use quote::quote;
// use quote::TokenStreamExt;

// Helper to parse attributes

fn parse_attr<T>(input: TokenStream2, keyword: &str) -> parse::Result<(T, TokenStream)>
where
    T: Parse,
{
    (|input: ParseStream<'_>| -> parse::Result<(T, TokenStream)> {
        let mut ts = vec![];
        let mut t: Option<T> = None;
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            let expr: Expr = input.parse()?;

            match &*ident.to_string() {
                keyword => {
                    println!("id found");
                    if t.is_some() {
                        return Err(parse::Error::new(ident.span(), "Already defined"));
                    } else {
                        let mut e = quote! {#expr};
                        let p = syn::parse::<T>(e.into())?;
                        println!("here ---------");
                        t = Some(p)
                    }
                }
                _ => {
                    ts.push((ident, expr));
                }
            }
        }
        let ts = quote! {plepps};
        match t {
            Some(t) => Ok((t, ts.into())),
            _ => Err(parse::Error::new(
                input.span(),
                format!("{} not found", keyword),
            )),
        }
    })
    .parse2(input)
}

#[test]
fn test_parse_attr() {
    let q = quote! {passes = pass1 };
    let r: parse::Result<(Ident, TokenStream)> = parse_attr(q, "passes");

    // println!("ok {:?}", r.0);
}
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

/// parse
///
/// input:
/// attr:   TokenStream2 representing the attributes for the 'app'
/// item:   TokenStream2 representing the body of the associated 'mod'
///
///
fn parse(attr: TokenStream2, item: TokenStream2) -> Result<TokenStream2, syn::parse::Error> {
    let mut attrs: Attr = syn::parse2(attr)?;
    let module: Module = syn::parse2(item)?;
    let mut next_pass = None; // the next pass to process
    let mut other_passes = vec![]; // a vector of remaining passes
    let mut other_attrs = vec![]; // a vector of other attributes for the 'app'
    for (id, e) in &mut attrs.attrs {
        match &*id.to_string() {
            "passes" => match e {
                Expr::Array(a) => {
                    let a = a.elems.clone();

                    let q = quote! {#a};
                    let mut idents = syn::parse::<CommaSep<Ident>>(q.into())?;

                    next_pass = idents.remove_first();
                    other_passes = idents.elems;
                }
                _ => panic!("RTIC ICE"),
            },
            _ => other_attrs.push((id, e)),
        }
    }

    // here we should report error instead of panic
    let next_pass = next_pass.unwrap();
    let ident = module.ident;
    let items = module.items;

    // code generation
    let mut attrs = vec![];
    for (id, expr) in other_attrs {
        attrs.push(quote! {#id = #expr,});
    }

    // generate code for the module
    //
    let ts = quote! {
        #[ #next_pass(passes = [#(#other_passes)*], #(#attrs)*)]
        mod #ident {

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

        let mut attrs = vec![];
        for ea in pun {
            let l = ea.left;
            let l = quote! {#l};

            let id = syn::parse::<Ident>(l.into())?;
            attrs.push((id, *ea.right));
        }

        Ok(Attr { attrs })
    }
}

/// Module
///
/// Contains a vector of syn::Item:s

pub(crate) struct Module {
    pub ident: Ident,
    pub items: Vec<Item>,
}

/// Parse for Module
///
/// Input:
/// The token stream representing the 'app' mod.
///
/// Output:
/// A Module consisting of a vector of syn::Items
///
/// Error: Returns parse error if the 'app' mod cannot be parsed by syn.
///
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
        let ident: Ident = input.parse()?;
        // bad this might panic
        let _brace_token: Brace = braced!(content in input);
        let items = content.call(parse_items)?;

        Ok(Module { ident, items })
    }
}

/// app
///
/// The top level procedural macro
///
/// Input:
///
/// attr:   TokenStream (internal Rust) representation of the 'app' attributes.
///
/// item:   TokenStream (internal Rust) representation of the item ('mod _ {}').
///
/// Error:  Generates compile error in case parsing fails.
///
/// Trace:  Generates 'target/app.rs' reflecting the expanded code.
///
/// Panic:  Generates Rust 'panic(RTIC ICE..)' in case 'target' directory missing.
#[proc_macro_attribute]
pub fn app(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ts = match parse(attr.into(), item.into()) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    // Try to write the expanded code to disk
    if std::path::Path::new("target").exists() {
        fs::write("target/app.rs", ts.to_string()).ok();
    } else {
        panic!("RTIC ICE: no 'target' directory found");
    }

    ts.into()
}
