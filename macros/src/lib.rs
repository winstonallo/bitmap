use proc_macro::TokenStream;
use syn::parse_macro_input;

mod generator;
mod parser;

#[proc_macro]
pub fn bitmap(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as parser::BitmapInput);
    match generator::expand_bitmap(parsed) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
