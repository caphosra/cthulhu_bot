use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{parse_macro_input, FnArg, ImplItem, ImplItemMethod, ItemImpl};

/// Finds a function from impl.
fn find_function<'l>(item_impl: &'l mut ItemImpl, fn_name: &str) -> Option<&'l mut ImplItemMethod> {
    item_impl.items.iter_mut().find_map(|item| match item {
        ImplItem::Method(method) => {
            if method.sig.ident.to_string() == fn_name {
                Some(method)
            } else {
                None
            }
        }
        _ => None,
    })
}

/// Specifies whether the command depends on the database.
#[proc_macro_attribute]
pub fn db_required(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: proc_macro2::TokenStream = attr.into();
    let mut item_impl = parse_macro_input!(item as ItemImpl);

    match find_function(&mut item_impl, "execute") {
        Some(function) => {
            // When the command doesn't depend on the database.
            if attr.to_string() == "false" {
                // Add a dummy argument.
                let arg: TokenStream = quote! {
                    _data: &Mutex<crate::database::SizedBotDatabase>
                }
                .into();

                function.sig.inputs.push(parse_macro_input!(arg as FnArg));
            }
        }
        None => panic!("A function `execute` should be implemented."),
    }

    let use_db_function: TokenStream = (quote! {
        fn use_db(&self) -> bool {
            #attr
        }
    })
    .into();

    item_impl
        .items
        .push(parse_macro_input!(use_db_function as ImplItem));

    item_impl.to_token_stream().into()
}

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
