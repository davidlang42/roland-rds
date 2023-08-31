use crate::helpers::get_array_len;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{Ident, Field, Generics};

pub fn impl_from_parameters(name: &Ident, fields: &Vec<&Field>, generics: &Generics) -> TokenStream {
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

pub fn impl_parameters(name: &Ident, fields: &Vec<&Field>, generics: &Generics) -> TokenStream {
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
