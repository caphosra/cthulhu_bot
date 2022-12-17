use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, GenericParam, ItemFn, WherePredicate};

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
