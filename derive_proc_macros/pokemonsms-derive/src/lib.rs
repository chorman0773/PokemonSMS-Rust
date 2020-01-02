extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

mod impl_derives{
    use proc_macro::{TokenStream};
    use syn::{Ident,Data};

    pub(crate) fn derive_binaryread(ast: syn::DeriveInput) -> TokenStream{
        let name :&Ident = &ast.ident;
        let data: &Data = &ast.data;
        let mut stream = TokenStream::new();
        match data{
            Data::Struct(s) =>{

            },
            Data::Enum(e) =>{

            },
            Data::Union(u) => {
                panic!("Cannot derive BinaryIOReadable for a union type");
            }
        };
        stream
    }
}

#[proc_macro_derive(BinaryIOReadable)]
pub fn binaryioreadable_derive(input: TokenStream) -> TokenStream{
    let ast = syn::parse(input).unwrap();
    impl_derives::derive_binaryread(ast)
}
