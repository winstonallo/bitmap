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

        let size = match ty.to_string().as_str() {
            "u1" => 1,
            "u2" => 2,
            "u3" => 3,
            "u4" => 4,
            "u5" => 5,
            "u6" => 6,
            "u7" => 7,
            _ => return Err(syn::Error::new_spanned(ty, "Expected one of u1, u2, u3, u4, u5, u6, and u7")),
        };

        Ok(FieldDef { name, size })
    }
}
