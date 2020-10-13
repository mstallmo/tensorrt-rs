use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Layer)]
pub fn layer_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_layer_derive(&ast)
}

fn impl_layer_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl private::LayerPrivate for #name {
            fn get_internal_layer(&self) -> *mut tensorrt_sys::Layer_t {
                self.internal_layer
            }
        }

        impl Layer for #name {}
    };

    gen.into()
}
