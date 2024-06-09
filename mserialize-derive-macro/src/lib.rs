extern crate core;

use quote::quote;

fn impl_mserialize_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl mserialize::MSerialize for #name {
            fn to_bytes_representation(&self) -> Box<[u8]> {
                bincode::encode_to_vec(self, bincode::config::standard().with_big_endian()).unwrap().into_boxed_slice()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(MSerialize)]
pub fn mserialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_mserialize_macro(&ast)
}

