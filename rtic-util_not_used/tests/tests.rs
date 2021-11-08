use proc_macro2::TokenStream as TokenStream2;
// use quote::quote;
use rtic_util;

#[test]
fn test_parse_attr() {
    let q = "
        #[passes = [pass1, pass2], plepps = d ]
        #[pas = [pass1, pass2], plepps = d ]
        ";
    let q: TokenStream2 = q.parse().unwrap();

    let v = rtic_util::app_attr(q).unwrap();

    println!("ok {:?}", v);
}
