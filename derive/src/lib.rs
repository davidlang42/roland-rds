extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{Ident, Body, Field, Ty, ConstExpr, Lit};

#[proc_macro_derive(Parameters)]
pub fn parameters(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let name = &ast.ident;
    if let Body::Struct(syn::VariantData::Struct(fields)) = &ast.body {
        let from_parameters = impl_from_parameters(name, fields);
        let parameters = impl_parameters(name, fields);

        let gen = quote! {
            #from_parameters
            #parameters
        };
    
        // Return the generated impl
        gen.parse().unwrap()
    } else {
        // Invalid use of macro
        panic!("Parameters can only derive on Structs with fields.");
    }
}

fn impl_from_parameters(name: &Ident, fields: &Vec<Field>) -> quote::Tokens {
    let mut size: usize = 0;
    let mut inner = Tokens::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        if let Ty::Array(_, ConstExpr::Lit(Lit::Int(len, _))) = &field.ty {
            // expects an Array of Parameter
            inner.append(quote! {
                #field_name: p.collect::<Vec<_>>().try_into().unwrap(),
            });
            size += *len as usize;
        } else {
            // expects a type which implements From<Parameter>
            inner.append(quote! {
                #field_name: p.next().unwrap().into(),
            });
            size += 1;
        }
    }
    quote! {
        impl From<[Parameter; #size]> for #name {
            fn from(value: [Parameter; #size]) -> Self {
                let mut p = value.into_iter();
                Self {
                    #inner
                }
            }
        }
    }
}

fn impl_parameters(name: &Ident, fields: &Vec<Field>) -> quote::Tokens {
    let mut size: usize = 0;
    let mut inner = Tokens::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        if let Ty::Array(_, ConstExpr::Lit(Lit::Int(len, _))) = &field.ty {
            // expects an Array of Parameter
            inner.append(quote! {
                for element in self.#field_name.iter() {
                    p.push(*element);
                }
            });
            size += *len as usize;
        } else {
            // expects a type which implements Into<Parameter>
            inner.append(quote! {
                p.push(self.#field_name.into());
            });
            size += 1;
        }
    }
    quote! {
        impl Parameters<#size> for #name {
            fn parameters(&self) -> [Parameter; #size] {
                let mut p: Vec<Parameter> = Vec::new();
                #inner
                p.try_into().unwrap()
            }
        }
    }
}
