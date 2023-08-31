extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod helpers;
mod parameters;

use helpers::get_named_fields;
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