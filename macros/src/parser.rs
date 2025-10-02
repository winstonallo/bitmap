use syn::punctuated::Punctuated;
use syn::token::Struct;
use syn::{Data, DataStruct, DeriveInput, Fields, Type, TypePath};
use syn::{
    Ident, Result, Token, braced,
    parse::{Parse, ParseStream},
};

pub struct BitmapInput {
    pub name: Ident,
    pub fields: Vec<FieldDef>,
}

impl Parse for BitmapInput {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Struct>()?;
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let punctuation: Punctuated<FieldDef, Token![,]> = content.parse_terminated(FieldDef::parse, Token![,])?;
        let fields = punctuation.into_iter().collect();
        Ok(BitmapInput { name, fields })
    }
}

impl TryFrom<DeriveInput> for BitmapInput {
    type Error = syn::Error;

    fn try_from(value: DeriveInput) -> std::result::Result<Self, Self::Error> {
        let name = value.ident;

        let fields = match value.data {
            Data::Struct(DataStruct {
                fields: Fields::Named(fields), ..
            }) => fields
                .named
                .into_iter()
                .map(|field| {
                    let field_name = field.ident.expect("Named field should have an ident");
                    let size = extract_size_from_type(&field.ty)?;
                    Ok(FieldDef { name: field_name, size })
                })
                .collect::<Result<Vec<_>>>()?,
            _ => return Err(syn::Error::new_spanned(name, "bitmap attribute can only be used on structs with named fields")),
        };

        Ok(BitmapInput { name, fields })
    }
}

pub struct FieldDef {
    pub name: Ident,
    pub size: u8,
}

impl Parse for FieldDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let ty: Ident = input.parse()?;
        Ok(FieldDef {
            name,
            size: parse_bit_width(&ty.to_string())?,
        })
    }
}

fn extract_size_from_type(ty: &Type) -> Result<u8> {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let segment = path.segments.last().unwrap();
            parse_bit_width(&segment.ident.to_string())
        }
        _ => Err(syn::Error::new_spanned(ty, "Expected a simple type like u1, u8, etc.")),
    }
}

fn parse_bit_width(ty: &str) -> Result<u8> {
    if !ty.starts_with("u") {
        return Err(syn::Error::new_spanned(ty, format!("Invalid type {ty}, expected u{{1..128}}")));
    }

    let size = match ty[1..].parse::<u8>() {
        Ok(val) => val,
        Err(e) => return Err(syn::Error::new_spanned(ty, format!("Could not parse type size: {e}"))),
    };

    if size == 0 || size > 128 {
        return Err(syn::Error::new_spanned(ty, format!("Invalid size for {ty}, expected u{{1..128}}")));
    }

    Ok(size)
}
