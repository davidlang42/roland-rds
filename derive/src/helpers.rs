use proc_macro2::Span;
use syn::{Field, Data, Type, DataStruct, Fields, FieldsNamed, TypeArray, Expr, ExprLit, Lit, Generics, GenericParam, LifetimeParam, Lifetime};

pub fn get_named_fields(data: &Data) -> Option<Vec<&Field>> {
    if let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), ..}) = data {
        Some(named.iter().collect())
    } else {
        None
    }
}

pub fn get_array_len(ty: &Type) -> Option<usize> {
    if let Type::Array(TypeArray { len: Expr::Lit(ExprLit { lit: Lit::Int(lit_int), ..}), .. }) = ty {
        lit_int.base10_parse().ok()
    } else {
        None
    }
}

pub fn insert_lifetime_param(original: &Generics, symbol: &str) -> Generics {
    let mut generics = original.clone();
    generics.params.insert(0, GenericParam::Lifetime(LifetimeParam::new(Lifetime::new(symbol, Span::call_site()))));
    generics
}
