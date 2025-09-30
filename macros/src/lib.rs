use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;  
use quote::quote;      
use quote::ToTokens;  

mod generator;
mod parser;

/// Generates a packed bitmap newtype struct with field-level bit access.
///
/// The macro expands to a newtype struct around a `u8` to `u128`, depending on the total bit width
/// of the definition, with automatically generated getters and setters for each field.
///
/// ### API
/// #### Usage Example
/// ```
/// use macros::bitmap;
///
/// bitmap!(
///     struct Player {
///         imposter: u1,
///         finished_tasks: u3,
///         kills: u3,
///     }
/// );
///
/// let mut player = Player(0);
/// assert_eq!(std::mem::size_of::<Player>(), 1);
///
/// player.set_imposter(1);
/// player.set_finished_tasks(5);
/// player.set_kills(3);
///
/// assert_eq!(player.imposter(), 1);
/// assert_eq!(player.finished_tasks(), 5);
/// assert_eq!(player.kills(), 3);
/// assert_eq!(*player, 0b01101011);
/// ```
/// #### Accessing fields
/// For each field `name: T`, where `T` is the smallest possible integer such that
/// `field_size <= integer.size`, `bitmap!` generates:
///
/// - `fn name(&self) -> T` — returns the value for `name`
/// - `fn set_name(&mut self, val: T)` — sets the value for `name`
///
/// #### Accessing the raw value
/// For the struct `Bits(T)`, where `T` is the unsigned integer type used for storage,
/// the following traits are implemented:
/// - `From<Bits> for T`
/// - `Deref for Bits`, with `fn deref(&self) -> T`
///
/// ```
/// use macros::bitmap;
///
/// bitmap!(
///     struct Bits {
///         a: u32,
///         b: u16,
///         c: u16,
///     }
/// );
///
/// let bits = Bits(0);
/// let underlying_u64: u64 = bits.into();
/// let underlying_u64 = *bits;
/// ```
/// ### Supported field types:
/// ```
/// use macros::bitmap;
///
/// bitmap!(
///     struct Bits {
///         flag: u1,
///         counter: u7,
///     }
/// );
/// ```
/// Each field must be in the form `uN`, where `1 <= N <= 128`.
/// ### Maximum total size
/// `bitmap!` uses the smallest possible integer type such that `total_bit_width <= integer.bit_width`.
/// The total bit width must fit into a `u128`. If you need more than that, consider using a `Vec`
/// of `bitmap`s.
/// ### Storage order
/// Fields are packed from **most significant bit (MSB)** to **least significant bit (LSB)**, matching
/// big-endian order.
///
/// This means the first declared field is stored in the highest bits of the underlying storage integer.
/// ```
/// use macros::bitmap;
///
/// bitmap!(
///     struct Bits {
///         a: u8,
///         b: u8,
///     }
/// );
///
/// let mut bits = Bits(0);
/// bits.set_a(0xaa)
///     .set_b(0xbb);
///
/// assert_eq!(*bits, 0xaabb);
/// ```
///
/// ### Note
/// `bitmap!` is built with hardware configuration in mind, where most packed bitmaps have a size
/// aligned to integer sizes. It does not use the _smallest possible size_: a bitmap with only one `u33`
/// field will take up 64 bits of space.
/// ```
/// use macros::bitmap;
///
/// bitmap!(
///     struct Bits {
///         field: u33,
///     }
/// );
///
/// assert_eq!(core::mem::size_of::<Bits>(), 8);
/// ```
///

#[proc_macro]
pub fn bitmap(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as parser::BitmapInput);
    match generator::expand_bitmap(parsed) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn bitmap_attr(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Convert the attribute macro input to the existing BitmapInput format
    let bitmap_input = convert_derive_to_bitmap_input(input);
    
    // Use the EXACT SAME expansion logic as the bitmap! macro
    match generator::expand_bitmap(bitmap_input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn convert_derive_to_bitmap_input(input: DeriveInput) -> parser::BitmapInput {
    let name = input.ident;  // Use 'name' not 'struct_name'
    
    // Extract struct fields
    let syn::Data::Struct(data_struct) = input.data else {
        panic!("#[bitmap_attr] can only be used on structs");
    };
    
    let syn::Fields::Named(fields_named) = data_struct.fields else {
        panic!("#[bitmap_attr] struct must have named fields");
    };
    
    // Convert each field to the format expected by the existing parser
    let fields = fields_named.named.into_iter().map(|field| {
        let field_name = field.ident.expect("Field must have a name");
        let field_type = field.ty;
        
        // Extract the size from the type (e.g., u1 -> 1, u7 -> 7)
        let type_str = field_type.to_token_stream().to_string();
        
        if !type_str.starts_with("u") {
            panic!("Field type must be unsigned integer like u1, u2, etc.");
        }
        
        let size: u8 = type_str[1..].parse().expect("Invalid bit width");
        
        if size == 0 || size > 128 {
            panic!("Invalid size for {}, expected u{{1..128}}", type_str);
        }
        
        parser::FieldDef {
            name: field_name,
            size,
        }
    }).collect();
    
    parser::BitmapInput {
        name,  // Use 'name' not 'struct_name'
        fields,
    }
}