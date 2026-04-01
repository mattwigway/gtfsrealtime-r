use proc_macro::TokenStream;
use quote::quote;

// Prost does not have a trait that allows us to generically access as_str_name(),
// so this trait just wraps the as_str_name() function with as_str_name_t(), which is a wrapper
// defined in the AsStrName trait in the main rust code.
#[proc_macro_derive(AsStrName)]
pub fn as_str_name_macro(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap(); // unwrap ok here, will panic at compile time
    let name = &ast.ident;
    let generated = quote! {
        impl crate::enums::AsStrName for #name {
            fn as_str_name_t(&self) -> &'static str {
                self.as_str_name()
            }
        }
    };

    generated.into()
}