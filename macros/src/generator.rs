use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;

use crate::parser::BitmapInput;

fn get_storage_ty(size: u8) -> TokenStream2 {
    match size {
        0..=8 => quote! { u8 },
        9..=16 => quote! { u16 },
        17..=32 => quote! { u32 },
        33..=64 => quote! { u64 },
        65..=128 => quote! { u128 },
        _ => unreachable!(),
    }
}

pub fn expand_bitmap(input: BitmapInput) -> syn::Result<TokenStream2> {
    let name = &input.name;
    let fields = &input.fields;
    let size: usize = input.fields.iter().map(|f| f.size as usize).sum();

    if size > 128 {
        return Err(syn::Error::new_spanned(name, "Too many fields: maximum supported size is 128 bits"));
    }

    let storage_ty = get_storage_ty(size as u8);

    let mut bit_index = size;
    let accessors = fields.iter().map(|ident| {
        bit_index -= ident.size as usize;
        let index: usize = bit_index;
        let setter_name = Ident::new(&format!("set_{}", ident.name), ident.name.span());
        let name = ident.name.to_owned();
        let size = ident.size;
        let this_storage_ty = get_storage_ty(size);
        let mask = if size != 128 {
            let mask_ty = get_storage_ty(size + 1);
            quote! { (((0b1 as #mask_ty) << #size) - 1) as #storage_ty }
        } else {
            quote! { 340282366920938463463374607431768211455 }
        };
        quote! {
            #[inline]
            pub const fn #name(&self) -> #this_storage_ty {
                ((self.0 >> #index) & #mask) as #this_storage_ty
            }

            #[inline]
            pub fn #setter_name(&mut self, val: #this_storage_ty) -> &mut Self {
                self.0 = ((self.0 & !((#mask) << #index)) | (((val as #storage_ty) & #mask) << #index));
                self
            }
        }
    });

    Ok(quote! {
        #[derive(Debug, Clone, Copy)]
        #[repr(transparent)]
         pub struct #name(#storage_ty);

         impl #name {
             #(#accessors)*
         }

         impl ::core::convert::From<#name> for #storage_ty {
             fn from(value: #name) -> Self {
                 value.0
             }
         }

         impl ::core::ops::Deref for #name {
             type Target = #storage_ty;
             fn deref(&self) -> &Self::Target {
                 &self.0
             }
         }
    })
}
