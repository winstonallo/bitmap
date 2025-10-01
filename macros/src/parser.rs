use syn::punctuated::Punctuated;
use syn::token::Struct;
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

pub struct FieldDef {
    pub name: Ident,
    pub size: u8,
}

impl Parse for FieldDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let ty: Ident = input.parse()?;

        let ty_str = ty.to_string();
        let ty_str = ty_str.as_str();
        if !ty_str.starts_with("u") {
            return Err(syn::Error::new_spanned(ty, format!("Invalid type {ty_str}, expected u{{1..128}}")));
        }
        let size = *match &ty_str[1..].parse::<u8>() {
            Ok(val) => val,
            Err(e) => return Err(syn::Error::new_spanned(ty, format!("Could not parse type size: {e}"))),
        };
        if size == 0 || size > 128 {
            return Err(syn::Error::new_spanned(ty, format!("Invalid size for {ty_str}, expected u{{1..128}}")));
        }

        Ok(FieldDef { name, size })
    }
}

pub fn parse_bit_width(ty: &syn::Ident) -> Result<u8> {
    let ty_str = ty.to_string();
    if !ty_str.starts_with("u") {
        return Err(syn::Error::new_spanned(ty, format!("Invalid type {ty_str}, expected u{{1..128}}")));
    }
    let size = ty_str[1..].parse::<u8>()
        .map_err(|e| syn::Error::new_spanned(ty, format!("Could not parse type size: {e}")))?;
    if size == 0 || size > 128 {
        return Err(syn::Error::new_spanned(ty, format!("Invalid size for {ty_str}, expected u{{1..128}}")));
    }
    Ok(size)
}
