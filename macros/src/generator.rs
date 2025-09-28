use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;

use crate::parser::BitmapInput;

fn get_packed_layout(size: usize) -> Vec<u8> {
    let usizes = [128, 64, 32, 16, 8];
    let mut remainder = size;
    let mut sizes = Vec::<u8>::new();

    for &usz in &usizes {
        while remainder >= usz as usize {
            sizes.push(usz);
            remainder -= usz as usize;
        }
    }

    if remainder > 0 {
        sizes.push(8);
    }

    sizes
}

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
    let _packed_layout = get_packed_layout(size);

    if size > 128 {
        return Err(syn::Error::new_spanned(name, "Too many fields: maximum supported size is 128 bits"));
    }

    let storage_ty = get_storage_ty(size as u8);

    let mut bit_index = 0;
    let mut storage_index = 0;
    let mut current_storage_ty_index = 0;
    let accessors = fields.iter().map(|ident| {
        if ident.size + current_storage_ty_index >= _packed_layout[storage_index] {
            // handle field spanning multiple storage units
        }
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
