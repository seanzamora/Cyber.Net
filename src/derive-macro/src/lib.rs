use proc_macro::TokenStream;
use quote::quote;
// use serde::{Deserialize, Serialize};
use syn::{parse, DeriveInput};

#[proc_macro_derive(Bincode)]
pub fn bincode(input: TokenStream) -> TokenStream {
    let input = parse::<DeriveInput>(input).unwrap();
    let name = input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics #name #type_generics #where_clause
        {
            fn serialize(&self) -> Vec<u8> {
                bincode::serialize(&self).unwrap()
            }
            fn deserialize(input: Vec<u8>) -> #name #type_generics
            {
                bincode::deserialize(&input[..]).unwrap()
            }
        }
    };
    gen.into()
}
