extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Object)]
pub fn derive_object(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_object(&ast)
}

fn impl_object(ast: &syn::DeriveInput) -> TokenStream {
    // Get the type of the object
    let name = &ast.ident;
    let object_type = name.to_string().to_lowercase();

    quote! {
        impl Object for #name {
            fn children(&self) -> Vec<&dyn Object> {
                vec![self]
            }

            fn mv(&mut self, other: &mut dyn Object) {
                
            }

            fn object_type(&self) -> &str {
                #object_type
            }

            fn read(&mut self) {

            }

            fn write(&self) {

            }
        }
    }.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
