use bitstruct::traits::*;

fn main() {
    let mut x: u128 = 0;
    x.set_bits(0..2, 0b11);
    assert_eq!(x, 0b11);
}
