use proc_macro::TokenStream;

mod data_accessor;

#[proc_macro_derive(U16FieldAccessor)]
pub fn u16_field_accessor_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    data_accessor::u16_field_accessor::macro_impl(&ast)
}
