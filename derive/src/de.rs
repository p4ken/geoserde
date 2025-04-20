use proc_macro2::TokenStream;
use quote::quote;
use syn::Data;

pub fn derive_geo_deserialize_2(input: TokenStream) -> TokenStream {
    let input = syn::parse2::<syn::DeriveInput>(input).unwrap();
    let struct_name = &input.ident;

    let data = match &input.data {
        Data::Struct(data) => data,
        _ => panic!("Feature can only be derived for structs"),
    };

    let mut geometry_field = None;
    let mut props_fields = Vec::new();
    let mut props_inits = Vec::new();

    for field in &data.fields {
        let ident = field.ident.as_ref().expect("Expected named field");
        let ty = &field.ty;
        let has_geometry = field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("geometry"));

        if has_geometry {
            if geometry_field.is_some() {
                panic!("Only one field can have #[geometry]");
            }
            geometry_field = Some(ident.clone());
        } else {
            props_fields.push(quote! { #ident: #ty });
            props_inits.push(quote! { #ident: __props.#ident });
        }
    }

    let geom_type = match geometry_field {
        Some(_) => quote! { _ },
        None => quote! { () },
    };

    let geom_binding = match geometry_field {
        Some(geom) => quote! { #geom: __geom, },
        None => quote! {},
    };

    let output = quote! {
        impl geoserde::DeserializeFeature for #struct_name {
            fn deserialize_feature(fmt: impl geoserde::ParseFeature) -> Self {
                #[derive(geoserde::serde::Deserialize)]
                struct __Properties {
                    #(#props_fields,)*
                }
                let (__geom, __props) = fmt.parse_feature::<#geom_type, __Properties>();
                Self {
                    #geom_binding
                    #(#props_inits,)*
                }
            }
        }
    };
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let input = quote! {
            #[derive(Feature)]
            struct MyStruct {
                #[geometry]
                my_geom: geo_types::Point,
                my_prop: i32,
            }
        };

        let expected = quote! {
        impl geoserde::DeserializeFeature for MyStruct {
            fn deserialize_feature(fmt: impl geoserde::ParseFeature) -> Self {
                #[derive(geoserde::serde::Deserialize)]
                struct __Properties {
                    my_prop: i32,
                }
                let (__geom, __props) = fmt.parse_feature::<_, __Properties>();
                Self {
                    my_geom: __geom,
                    my_prop: __props.my_prop,
                }
            }
        }
            };

        let actial = derive_geo_deserialize_2(input);
        assert_eq!(actial.to_string(), expected.to_string());
    }

    #[test]
    fn test_no_geom() {
        let input = quote! {
            #[derive(Feature)]
            struct MyStruct {
                my_prop: i32,
            }
        };

        let expected = quote! {
        impl geoserde::DeserializeFeature for MyStruct {
            fn deserialize_feature(fmt: impl geoserde::ParseFeature) -> Self {
                #[derive(geoserde::serde::Deserialize)]
                struct __Properties {
                    my_prop: i32,
                }
                let (__geom, __props) = fmt.parse_feature::<(), __Properties>();
                Self {
                    my_prop: __props.my_prop,
                }
            }
        }
            };

        let actial = derive_geo_deserialize_2(input);
        assert_eq!(actial.to_string(), expected.to_string());
    }
}
