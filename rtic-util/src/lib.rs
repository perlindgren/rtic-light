use std::collections::btree_map::ValuesMut;

use syn::{
    braced, /* parenthesized, */
    parse::{self, Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Brace,
    Attribute, Expr, ExprAssign, Ident, Item, Result, /*  LitBool, LitInt, Path*/ Token,
};

use proc_macro2::{Punct, Span, TokenStream as TokenStream2};

use quote::{quote, ToTokens};

// pub fn parse_attr<'a, T>(
//     input: ParseStream<'a>,
//     keyword: &str,
// ) -> parse::Result<(T, ParseStream<'a>)>
// where
//     T: Parse,
// {
//     let mut ts = vec![];
//     let mut t: Option<T> = None;
//     while !input.is_empty() {
//         let ident: Ident = input.parse()?;
//         let _: Token![=] = input.parse()?;
//         let expr: Expr = input.parse()?;

//         match &*ident.to_string() {
//             keyword => {
//                 println!("id found");
//                 if t.is_some() {
//                     return Err(parse::Error::new(ident.span(), "Already defined"));
//                 } else {
//                     let mut e = quote! {#expr};
//                     let p = syn::parse::<T>(e.into())?;
//                     println!("here ---------");
//                     t = Some(p)
//                 }
//             }
//             _ => {
//                 ts.push((ident, expr));
//             }
//         }
//     }

//     match t {
//         Some(t) => Ok((t, input)),
//         _ => Err(parse::Error::new(
//             input.span(),
//             format!("{} not found", keyword),
//         )),
//     }
// }

#[derive(Debug)]
pub struct Attrs {
    attrs: Vec<Attribute>,
}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Attrs {
            attrs: input.call(Attribute::parse_outer)?,
        })
    }
}

impl<T> Parse for BracedVec<T>
where
    T: Parse,
{
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _brace_token: Brace = braced!(content in input);
        let p: Punctuated<T, Token![,]> = content.parse_terminated(T::parse)?;
        let vec: Vec<T> = p.into_iter().map(|v| v).collect();
        Ok(BracedVec { vec })
    }
}

#[derive(Debug)]
struct IdT<T>
where
    T: Parse + Clone,
{
    id: Ident,
    expr: T,
}

impl<T> Parse for IdT<T>
where
    T: Parse + Clone,
{
    fn parse(input: ParseStream) -> Result<Self> {
        let id: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let expr: T = input.parse()?;
        Ok(IdT { id, expr })
    }
}

#[derive(Debug)]
pub struct AppAttr {
    // next_pass: Ident,
    other_attrs: Vec<Attribute>,
}

#[derive(Debug)]
struct BracedVec<T>
where
    T: Parse,
{
    vec: Vec<T>,
}
pub fn app_attr(input: TokenStream2) -> Result<AppAttr> {
    let attrs: Attrs = syn::parse2(input)?;
    let mut other_attrs = vec![];
    for attr in attrs.attrs {
        println!("attr {:?}", attr.tokens);
        println!("----------");

        let i: Punctuated<IdT<Ident>, Token![,]> =
            attr.parse_args_with(Punctuated::parse_terminated)?;
        //  = attr.parse_args().unwrap();
        println!("i {:?}", i);

        // let at : Parse::parse2(Attribute::parse_outer, attr.tokens)?;
        // let p: Punctuated<IdT<Ident>, Token![,]> =
        //     attr.parse_args_with(Punctuated::parse_terminated)?;
        // println!("p {:?}", p);

        // for ide in p {
        //     match &*ide.id.to_string() {
        //         "passes" => {
        //             println!("passes found");
        //             println!("expr {:?}", ide.expr);
        //             // let t = ide.expr.to_token_stream();
        //             // let v: BracedVec<Ident> = syn::parse2(t)?;
        //             // println!("v {:?}", v);
        //         }
        //         _ => {}
        //     }
        // }

        // println!();
        other_attrs.push(attr);
    }

    Ok(AppAttr {
        // next_pass: Ident::new(&"hello", syn::span::new()),
        other_attrs,
    })
}

struct IdExpr(Vec<IdT<Expr>>);

impl Parse for IdExpr {
    fn parse(input: ParseStream) -> Result<IdExpr> {
        let attrs: Attrs = input.parse()?;
        let mut data = vec![];

        for attr in attrs.attrs {
            let res: Punctuated<IdT<Expr>, Token![,]> =
                attr.parse_args_with(Punctuated::parse_terminated)?;

            for pair in res.into_pairs() {
                data.push(pair.into_value())
            }
        }
        Ok(IdExpr(data))
    }
}

fn attr<T>(attrs: IdExpr, keyword: &str, span: Span) -> Result<T>
where
    T: Parse,
{
    let mut res = None;
    for attr in attrs.0 {
        if keyword == attr.id.to_string() {
            let t = attr.expr.to_token_stream();

            let p: T = syn::parse2(t.clone())?;
            if res.is_some() {
                return Err(parse::Error::new(t.span(), "Already defined"));
            } else {
                res = Some(p);
            }
        }
    }
    match res {
        Some(p) => Ok(p),
        _ => Err(parse::Error::new(
            span,
            format!("Keyword {} not found", keyword),
        )),
    }
}

#[test]
fn test_parse_attr() {
    let q: TokenStream2 = quote!(#[task(priority = 5, task_local = [plepps])]);

    let v: IdExpr = syn::parse2(q.clone()).unwrap();
    let a: Result<syn::LitInt> = attr(v, "task_local", q.span());

    println!("a {:?}", a);
}

#[test]
fn test_parse_attr2() {
    let q: TokenStream2 = quote!(#[task(priority = 5, task_local = [plepps])]);

    let v: IdExpr = syn::parse2(q.clone()).unwrap();
    let a: Result<syn::LitInt> = attr(v, "priority", q.span());

    println!("a {:?}", a);
}

#[test]
fn test_parse_attr3() {
    let q: TokenStream2 = quote!(#[task(priority = 5, task_local = [plepps])]);

    let v: IdExpr = syn::parse2(q.clone()).unwrap();
    let a: Result<syn::LitInt> = attr(v, "priorit", q.span());

    println!("a {:?}", a);
}

#[test]
fn test_parse_attr4() {
    let q: TokenStream2 = quote!(#[task(priority = 5, priority = 7)]);

    let v: IdExpr = syn::parse2(q.clone()).unwrap();
    let a: Result<syn::LitInt> = attr(v, "priority", q.span());

    println!("a {:?}", a);
}

#[test]
fn test_app_attr() {
    let q: TokenStream2 = quote!(#[app(passes = [pass1, pass2], peripherals = true)]);
    let q: TokenStream2 = quote!(#[app(id = id2)]);

    let v = app_attr(q); // .unwrap();

    // println!("ok {:?}", v);
}
