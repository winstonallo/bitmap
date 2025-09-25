use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;

use crate::parser::BitmapInput;

pub fn expand_bitmap(input: BitmapInput) -> syn::Result<TokenStream2> {
    let name = &input.name;
    let fields = &input.fields;
    let size = input.fields.iter().map(|f| f.size).sum();

    if size > 64 {
        return Err(syn::Error::new_spanned(name, "Too many fields: max supported size is 64 bits"));
    }

    let storage_ty = match size {
        0..=8 => quote! { u8 },
        9..=16 => quote! { u16 },
        17..=32 => quote! { u32 },
        33..=64 => quote! { u64 },
        _ => unreachable!(),
    };

    let mut bit_index = 0;
    let accessors = fields.iter().map(|ident| {
        let index: u8 = bit_index;
        bit_index += ident.size;
        let setter_name = Ident::new(&format!("set_{}", ident.name), ident.name.span());
        let name = ident.name.to_owned();
        let size = ident.size;
        let mask = quote! { ((0b1 << #size) - 1) as #storage_ty };
        quote! {
            pub fn #name(&self) -> #storage_ty {
                (self.0 >> #index) & #mask
            }

            pub fn #setter_name(&mut self, val: u8) -> &mut Self {
                self.0 = ((self.0 & !((#mask) << #index)) | (((val as #storage_ty) & #mask) << #index));
                self
            }
        }
    });

    Ok(quote! {
        #[derive(Debug, Clone, Copy)]
        #[repr(transparent)]
         pub struct #name(pub #storage_ty);

         impl #name {
             #(#accessors)*
         }
    })
}
