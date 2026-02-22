use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{parse_macro_input, ImplItem, ItemImpl};

/// Specifies whether the command depends on the database.
#[proc_macro_attribute]
pub fn naming(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_impl = parse_macro_input!(item as ItemImpl);
    let self_type = item_impl.self_ty.to_token_stream().to_string();

    let regex = Regex::new(r"^(.*)Command$").unwrap();
    let name = format!(
        "\"{}\"",
        regex.captures(&self_type).unwrap().get(1).unwrap().as_str()
    );

    let name: proc_macro2::TokenStream = name.to_lowercase().parse().unwrap();

    let name_function: TokenStream = (quote! {
        fn name(&self) -> &str {
            #name
        }
    })
    .into();

    item_impl
        .items
        .push(parse_macro_input!(name_function as ImplItem));

    item_impl.to_token_stream().into()
}
