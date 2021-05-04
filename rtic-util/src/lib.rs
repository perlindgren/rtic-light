use syn::{
    braced, /* parenthesized, */
    parse::{self, Parse, ParseStream, Parser},
    punctuated::Punctuated,
    token::Brace,
    Attribute, Expr, ExprAssign, Ident, Item, Result, /*  LitBool, LitInt, Path*/ Token,
};

use proc_macro2::TokenStream as TokenStream2;

use quote::quote;

pub fn parse_attr<'a, T>(
    input: ParseStream<'a>,
    keyword: &str,
) -> parse::Result<(T, ParseStream<'a>)>
where
    T: Parse,
{
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

    match t {
        Some(t) => Ok((t, input)),
        _ => Err(parse::Error::new(
            input.span(),
            format!("{} not found", keyword),
        )),
    }
}

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


#[derive(Debug)]
pub struct AppAttr {
    // next_pass: Ident,
    other_attrs: Vec<Attribute>,
}

pub fn app_attr(input: TokenStream2) -> Result<AppAttr> {
    let attrs: Attrs = syn::parse2(input)?;
    let mut other_attrs = vec![];
    for attr in attrs.attrs {
        let q = quote! {#attr};
        println!("q {:?}", q);
        println!("---------");
        println!("ts {:?}", attr.tokens);
        println!("---------");
        let e = syn::parse2::<Vec<ExprAssign>>(attr.tokens.clone())?;
        let e = syn::parse2::<Vec<ExprAssign>>(attr.tokens.clone())?;

        // println!("e.left :{:?}", e.left);
        // println!("e.right :{:?}", e.right);

        println!();
        other_attrs.push(attr);
    }

    Ok(AppAttr {
        // next_pass: Ident::new(&"hello", syn::span::new()),
        other_attrs,
    })
}
