extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{Ident, Body, Field};

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
        panic!("Parameters can only derive on Structs with fields.");
    }
}

fn impl_from_parameters(name: &Ident, fields: &Vec<Field>) -> quote::Tokens {
    let size = fields.len();
    let mut inner = Tokens::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        inner.append(quote! {
            #field_name: p.next().unwrap().into(),
        });
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
    let size = fields.len();
    let mut inner = Tokens::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        inner.append(quote! {
            p.push(self.#field_name.into());
        });
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


// impl Parameters<20> for Gm2ChorusParameters {
//     fn parameters(&self) -> [Parameter; 20] {
//         let mut p: Vec<Parameter> = Vec::new();
//         p.push(self.pre_lpf.into());
//         p.push(self.level.into());
//         p.push(self.feedback.into());
//         p.push(self.delay.into());
//         p.push(self.rate.into());
//         p.push(self.depth.into());
//         p.push(self.send_to_reverb.into());
//         for unused_parameter in self.unused_parameters.iter() {
//             p.push(*unused_parameter);
//         }
//         p.try_into().unwrap()
//     }
// }

// impl Default for Gm2ChorusParameters {
//     fn default() -> Self {
//         Self {
//             pre_lpf: PreLpf(0),
//             level: Level(64),
//             feedback: Level(8),
//             delay: Level(80),
//             rate: Level(3),
//             depth: Level(19),
//             send_to_reverb: Level(0),
//             unused_parameters: Default::default()
//         }
//     }
// }
