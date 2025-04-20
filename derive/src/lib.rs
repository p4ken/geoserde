mod de;

use proc_macro::TokenStream;

const _: () = {
    #[cfg(not(feature = "global_attribute"))]
    compile_error!("currently, global_attribute feature cannot be disabled");
};

// geoserde だから derive 名も接頭辞 geo が最も予測しやすい
#[proc_macro_derive(GeoSerialize, attributes(serde, geoserde, geometry))]
pub fn derive_geo_serialize(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(GeoDeserialize, attributes(serde, geoserde, geometry))]
pub fn derive_geo_deserialize(input: TokenStream) -> TokenStream {
    de::derive_geo_deserialize_2(input.into()).into()
}
