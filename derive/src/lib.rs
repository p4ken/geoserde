use proc_macro::TokenStream;
use quote::quote;

#[cfg(not(feature = "global_attribute"))]
#[proc_macro_derive(Feature, attributes(serde, geoserde))]
pub fn derive_feature(item: TokenStream) -> TokenStream {
    eprintln!("item: {:#?}", &item);
    TokenStream::new()
}

#[cfg(feature = "global_attribute")]
#[proc_macro_derive(Feature, attributes(serde, geoserde, geometry))]
pub fn derive_feature(_item: TokenStream) -> TokenStream {
    // eprintln!("item: {:#?}", &item);
    let q = quote! {
        struct B;
        impl B{fn b() { println!("hello"); }}
    }
    .into();
    q
}

#[proc_macro_derive(GeoSerialize, attributes(serde, geoserde, geometry))]
pub fn derive_ser_feature(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(GeoDeserialize, attributes(serde, geoserde, geometry))]
pub fn derive_de_feature(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
