extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod helpers;
mod parameters;

use helpers::{get_named_fields, insert_lifetime_param};
use parameters::{impl_from_parameters, impl_parameters};
use syn::DeriveInput;

#[proc_macro_derive(Parameters)]
pub fn parameters(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = syn::parse_macro_input!(input as DeriveInput);

    // Build the impl
    if let Some(fields) = get_named_fields(&ast.data) {
        let name = &ast.ident;

        let from_parameters = impl_from_parameters(name, &fields, &ast.generics);
        let parameters = impl_parameters(name, &fields, &ast.generics);
    
        // Return the generated impls
        quote! {
            #from_parameters
            #parameters
        }.into()
    } else {
        // Invalid use of macro
        panic!("Parameters can only derive on Structs with fields.");
    }
}

#[proc_macro_derive(EnumParameter)]
pub fn enum_parameter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = syn::parse_macro_input!(input as DeriveInput);

    // Build the impl
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics From<Parameter> for #name #ty_generics #where_clause {
            fn from(value: Parameter) -> Self {
                Self::iter().nth(value.0 as usize).expect(&format!("Invalid {}: Parameter({})", type_name_pretty::<Self>(), value.0))
            }
        }
        
        impl #impl_generics Into<Parameter> for #name #ty_generics #where_clause {
            fn into(self) -> Parameter {
                Parameter(Self::iter().position(|s| s == self).unwrap() as i16)
            }
        }
    }.into()
}

#[proc_macro_derive(DiscreteValuesSerialization)]
pub fn discrete_values_serialization(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = syn::parse_macro_input!(input as DeriveInput);

    // Build the impl
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let generics_with_de = insert_lifetime_param(&ast.generics, "'de");
    quote! { 
        impl #impl_generics JsonSchema for #name #ty_generics #where_clause {
            fn schema_name() -> String {
                type_name_pretty::<Self>().into()
            }

            fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                enum_schema(Self::values().into_iter().map(Self::format).collect())
            }
        }

        impl #impl_generics Serialize for #name #ty_generics #where_clause {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer {
                Self::format(self.0).serialize(serializer)
            }
        }

        impl #generics_with_de Deserialize<'de> for #name #ty_generics #where_clause {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de> {
                let value: Value = Deserialize::deserialize(deserializer)?;
                match value {
                    Value::String(s) => {
                        for v in Self::values().into_iter() {
                            if s == Self::format(v) {
                                return Ok(Self(v));
                            }
                        }
                        Err(de::Error::custom(format!("String is not a valid discrete value: {}", s)))
                    }
                    _ => Err(de::Error::custom(format!("Expected string")))
                }
            }
        }

        impl #impl_generics From<Parameter> for #name #ty_generics #where_clause {
            fn from(parameter: Parameter) -> Self {
                let values = Self::values();
                if parameter.0 < Self::OFFSET || parameter.0 >= Self::OFFSET + values.len() as i16 {
                    panic!("Parameter out of range: {} (expected {}-{})", parameter.0, Self::OFFSET, Self::OFFSET + values.len() as i16 - 1)
                }
                Self(values.into_iter().nth((parameter.0 as i16 - Self::OFFSET) as usize).unwrap())
            }
        }

        impl #impl_generics Into<Parameter> for #name #ty_generics #where_clause {
            fn into(self) -> Parameter {
                if let Some(position) = Self::values().iter().position(|v| Self::equal(v, &self.0)) {
                    return Parameter(position as i16 + Self::OFFSET);
                } else {
                    panic!("Invalid discrete value: {}", self.0);
                }
            }
        }
    }.into()
}
