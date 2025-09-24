use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;

use crate::parser::BitmapInput;

pub fn expand_bitmap(input: BitmapInput) -> syn::Result<TokenStream2> {
    let name = &input.name;
    let fields = &input.fields;
    let num_fields = fields.len();

    if num_fields > 64 {
        return Err(syn::Error::new_spanned(name, "Too many fields: max supported is 64"));
    }

    let storage_ty = match num_fields {
        0..=8 => quote! { u8 },
        9..=16 => quote! { u16 },
        17..=32 => quote! { u32 },
        33..=64 => quote! { u64 },
        _ => unreachable!(),
    };

    let accessors = fields.iter().enumerate().map(|(i, ident)| {
        let index = i as u64;
        let setter_name = Ident::new(&format!("set_{ident}"), ident.span());

        quote! {
            pub fn #ident(&self) -> u8 {
                (self.0 >> #index) & 1
            }

            pub fn #setter_name(&mut self, val: bool) {
                if val {
                     self.0 |= 1 << #index;
                 } else {
                     self.0 &= !(1 << #index);
                 }
            }
        }
    });

    Ok(quote! {
        #[repr(transparent)]
         pub struct #name(pub #storage_ty);

         impl #name {
             #(#accessors)*
         }
    })
}
