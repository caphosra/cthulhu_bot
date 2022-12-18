use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{
    parse_macro_input, FnArg, GenericParam, ImplItem, ItemFn, ItemImpl, Stmt, WherePredicate,
};

/// Specifies whether the command depends on the database.
#[proc_macro_attribute]
pub fn db_required(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: proc_macro2::TokenStream = attr.into();

    let mut function = parse_macro_input!(item as ItemFn);

    // When the command doesn't depend on the database.
    if attr.to_string() == "false" {
        // Declare a new lifetime and swap the order of them.
        let new_lifetime = quote! {
            'new_lifetime
        };
        let new_lifetime_token: TokenStream = new_lifetime.clone().into();

        let async_trait_lifetime: TokenStream = quote! {
            'async_trait
        }
        .into();

        function.sig.generics.params.pop();
        function
            .sig
            .generics
            .params
            .push(parse_macro_input!(new_lifetime_token as GenericParam));
        function
            .sig
            .generics
            .params
            .push(parse_macro_input!(async_trait_lifetime as GenericParam));

        // Add a dummy argument.
        let arg: TokenStream = quote! {
            _data: &#new_lifetime Mutex<crate::database::SizedBotDatabase>
        }
        .into();

        function.sig.inputs.push(parse_macro_input!(arg as FnArg));

        // Update the where clause.
        let where_lifetime: TokenStream = (quote! {
            #new_lifetime: 'async_trait
        })
        .into();

        let mut where_clause = function.sig.generics.where_clause.clone().unwrap();
        where_clause
            .predicates
            .push(parse_macro_input!(where_lifetime as WherePredicate));
        function.sig.generics.where_clause = Some(where_clause);
    }

    quote! {
        fn use_db(&self) -> bool {
            #attr
        }

        #function
    }
    .into()
}

/// Specifies whether the command depends on the database.
#[proc_macro_attribute]
pub fn naming(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut impl_item = parse_macro_input!(item as ItemImpl);
    let self_type = impl_item.self_ty.to_token_stream().to_string();

    let regex = Regex::new(r"^(.*)Command$").unwrap();
    let name = format!(
        "\"{}\"",
        regex.captures(&self_type).unwrap().get(1).unwrap().as_str()
    );

    let name: proc_macro2::TokenStream = name.to_lowercase().parse().unwrap();

    for item in impl_item.items.iter_mut() {
        match item {
            ImplItem::Method(method) => {
                if method.sig.ident.to_string() == "register" {
                    let arg = match method.sig.inputs.last().unwrap() {
                        FnArg::Receiver(_) => panic!("The last argument must not be a receiver."),
                        FnArg::Typed(typed) => typed.pat.to_token_stream(),
                    };

                    let stmt: TokenStream = (quote! {
                        #arg.name(#name);
                    })
                    .into();

                    method
                        .block
                        .stmts
                        .insert(0, parse_macro_input!(stmt as Stmt));
                }
            }
            _ => {}
        }
    }

    let name_function: TokenStream = (quote! {
        fn name(&self) -> &str {
            #name
        }
    })
    .into();

    impl_item
        .items
        .push(parse_macro_input!(name_function as ImplItem));

    impl_item.to_token_stream().into()
}
