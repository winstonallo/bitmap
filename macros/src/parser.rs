use syn::punctuated::Punctuated;
use syn::token::Struct;
use syn::{
    Ident, Result, Token, braced,
    parse::{Parse, ParseStream},
};

const BITMAP_TYPES: [&'static str; 7] = ["u1", "u2", "u3", "u4", "u5", "u6", "u7"];

pub struct BitmapInput {
    pub name: Ident,
    pub fields: Vec<Ident>,
}

impl Parse for BitmapInput {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Struct>()?;
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let punctuation: Punctuated<FieldDef, Token![,]> = content.parse_terminated(FieldDef::parse, Token![,])?;
        let fields = punctuation.into_iter().map(|f| f.name).collect();
        Ok(BitmapInput { name, fields })
    }
}

struct FieldDef {
    name: Ident,
}

impl Parse for FieldDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let ty: Ident = input.parse()?;

        if !BITMAP_TYPES.contains(&ty.to_string().as_str()) {
            return Err(syn::Error::new_spanned(ty, "Expected one of u1, u2, u3, u4, u5, u6, and u7"));
        }

        Ok(FieldDef { name })
    }
}
