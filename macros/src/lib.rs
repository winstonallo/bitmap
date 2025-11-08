use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod generator;
mod parser;

/// Generates a packed bitstruct newtype struct with field-level bit access.
///
/// The macro expands to a newtype struct around a `u8` to `u128`, depending on the total bit width
/// of the definition, with automatically generated getters and setters for each field.
///
/// ### API
/// #### Usage Example
/// ```
/// use macros::bitstruct;
///
/// #[bitstruct]
/// struct Player {
///     imposter: u1,
///     finished_tasks: u3,
///     kills: u3,
/// }
///
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
/// `field_size <= integer.size`, `bitstruct` generates:
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
/// use macros::bitstruct;
///
/// #[bitstruct]
/// struct Bits {
///     a: u32,
///     b: u16,
///     c: u16,
/// }
///
/// let bits = Bits(0);
/// let underlying_u64: u64 = bits.into();
/// let underlying_u64 = *bits;
/// ```
/// ### Supported field types:
/// ```
/// use macros::bitstruct;
///
/// #[bitstruct]
/// struct Bits {
///     flag: u1,
///     counter: u7,
/// }
/// ```
/// Each field must be in the form `uN`, where `1 <= N <= 128`.
/// ### Maximum total size
/// `bitstruct` uses the smallest possible integer type such that `total_bit_width <= integer.bit_width`.
/// The total bit width must fit into a `u128`. If you need more than that, consider using a `Vec`
/// of `bitstruct`s.
/// ### Storage order
/// Fields are packed from **most significant bit (MSB)** to **least significant bit (LSB)**, matching
/// big-endian order.
///
/// This means the first declared field is stored in the highest bits of the underlying storage integer.
/// ```
/// use macros::bitstruct;
///
/// #[bitstruct]
/// struct Bits {
///     a: u8,
///     b: u8,
/// }
///
/// let mut bits = Bits(0);
/// bits.set_a(0xaa)
///     .set_b(0xbb);
///
/// assert_eq!(*bits, 0xaabb);
/// ```
///
/// ### Note
/// `bitstruct` is built with hardware configuration in mind, where most packed bitstructs have a size
/// aligned to integer sizes. It does not use the _smallest possible size_: a bitstruct with only one `u33`
/// field will take up 64 bits of space.
/// ```
/// use macros::bitstruct;
///
/// #[bitstruct]
/// struct Bits {
///     field: u33,
/// }
///
/// assert_eq!(core::mem::size_of::<Bits>(), 8);
/// ```
///
#[proc_macro_attribute]
pub fn bitstruct(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);
    match parser::BitmapInput::try_from(parsed) {
        Ok(bitstruct_input) => match generator::expand_bitstruct(bitstruct_input) {
            Ok(tokens) => tokens.into(),
            Err(err) => err.to_compile_error().into(),
        },
        Err(err) => err.to_compile_error().into(),
    }
}
