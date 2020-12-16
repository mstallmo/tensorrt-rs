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
            fn get_internal_layer(&self) -> *mut tensorrt_sys::nvinfer1_ILayer {
                self.internal_layer as *mut tensorrt_sys::nvinfer1_ILayer
            }
        }

        impl Layer for #name {}
    };

    gen.into()
}

#[proc_macro_derive(Dim)]
pub fn dim_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_dim_derive(&ast)
}

fn impl_dim_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl private::DimsPrivate for #name {
            fn get_internal_dims(&self) -> *mut tensorrt_sys::Dims_t {
                self.internal_dims
            }
        }

        impl Dim for #name {}
    };

    gen.into()
}
