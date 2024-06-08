extern crate core;

use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

fn impl_mserialize_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    println!("name = {}", name);
    let gen = quote! {
        {
        use std::net::TcpStream;
        use ::io::Write;
        impl MSerialize for #name {
            fn write_buff(&self, tcp_stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
                tcp_stream.write_all(&*self.to_bytes_representation())?;
                Ok(())
            }
            fn to_bytes_representation(&self) -> Box<[u8]> {
                let ::bincode::config = config::standard().with_big_endian()    ;
                bincode::encode_to_vec(self, config).unwrap().into_boxed_slice()
            }
        }
            }
    };
    gen.into()
}

#[proc_macro_derive(MSerialize)]
pub fn mserialize_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_mserialize_macro(&ast)
}