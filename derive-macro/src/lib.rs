use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Bincode)]
pub fn bincode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let gen = quote! {
        impl #name{
            fn serialize(&self) -> Vec<u8> {
                bincode::serialize(&self).unwrap()
            }
            fn deserialize(input: Vec<u8>) -> #name{
                bincode::deserialize(&input[..]).unwrap()
            }
        }
    };
    gen.into()
}
