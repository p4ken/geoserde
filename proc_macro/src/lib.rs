use proc_macro::TokenStream;

#[proc_macro_derive(Feature, attributes(geometry, serde))]
pub fn derive_feature(item: TokenStream) -> TokenStream {
    eprintln!("item: {:#?}", &item);
    TokenStream::new()
}
