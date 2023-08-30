extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{Ident, Field, DeriveInput, Data, Type, DataStruct, Fields, FieldsNamed, TypeArray, Expr, ExprLit, Lit, Generics};

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

fn get_named_fields(data: &Data) -> Option<Vec<&Field>> {
    if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), ..}) = data {
        Some(named.iter().collect())
    } else {
        None
    }
}

fn get_array_len(ty: &Type) -> Option<usize> {
    if let Type::Array(TypeArray { len: Expr::Lit(ExprLit { lit: Lit::Int(lit_int), ..}), .. }) = ty {
        lit_int.base10_parse().ok()
    } else {
        None
    }
}

fn impl_from_parameters(name: &Ident, fields: &Vec<&Field>, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut size: usize = 0;
    let mut inner = TokenStream::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        if let Some(len) = get_array_len(&field.ty) {
            // expects an Array of Parameter
            inner.append_all(quote! {
                #field_name: p.collect::<Vec<_>>().try_into().unwrap(),
            });
            size += len;
        } else {
            // expects a type which implements From<Parameter>
            inner.append_all(quote! {
                #field_name: p.next().unwrap().into(),
            });
            size += 1;
        }
    }
    quote! {
        impl #impl_generics From<[Parameter; #size]> for #name #ty_generics #where_clause {
            fn from(value: [Parameter; #size]) -> Self {
                let mut p = value.into_iter();
                Self {
                    #inner
                }
            }
        }
    }
}

fn impl_parameters(name: &Ident, fields: &Vec<&Field>, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut size: usize = 0;
    let mut inner = TokenStream::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        if let Some(len) = get_array_len(&field.ty) {
            // expects an Array of Parameter
            inner.append_all(quote! {
                for element in self.#field_name.iter() {
                    p.push(*element);
                }
            });
            size += len;
        } else {
            // expects a type which implements Into<Parameter>
            inner.append_all(quote! {
                p.push(self.#field_name.into());
            });
            size += 1;
        }
    }
    quote! {
        impl #impl_generics Parameters<#size> for #name #ty_generics #where_clause {
            fn parameters(&self) -> [Parameter; #size] {
                let mut p: Vec<Parameter> = Vec::new();
                #inner
                p.try_into().unwrap()
            }
        }
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