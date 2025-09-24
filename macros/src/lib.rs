use proc_macro::TokenStream;
use syn::parse_macro_input;

mod generator;
mod parser;

/// Generates a bitmap struct from the given definition.
///
/// The macro expands to a newtype struct around a `u8` to `u64` (depending on the total size
/// of the definition), with automatically generated getters and setters for each field.
///
/// ### Supported field types
/// - `u1` through `u7`
///
/// ### Current Limitations
/// - Total bit size must be ≤ 64 bits.
/// - No values larger than `u7` currently supported
///
/// ### Generated API
/// For each field `name: T`, the macro generates:
/// - `fn name(&self) -> T` — returns the field value.
/// - `fn set_name(&mut self, val: T)` — sets the field value.
///
/// ### Example
/// ```
/// # use macros::bitmap;
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
/// ```
#[proc_macro]
pub fn bitmap(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as parser::BitmapInput);
    match generator::expand_bitmap(parsed) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
