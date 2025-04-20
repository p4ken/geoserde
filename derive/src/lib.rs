mod de;

use proc_macro::TokenStream;
use quote::quote;

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

// geoserde だから derive 名も接頭辞 geo が最も予測しやすい
#[proc_macro_derive(GeoSerialize, attributes(serde, geoserde, geometry))]
pub fn derive_geo_serialize(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(GeoDeserialize, attributes(serde, geoserde, geometry))]
pub fn derive_geo_deserialize(input: TokenStream) -> TokenStream {
    de::derive_geo_deserialize_2(input.into()).into()
}
