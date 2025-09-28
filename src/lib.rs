pub use macros::bitmap;
pub use traits::*;

mod traits;

#[test]
fn one_bit() {
    bitmap!(
        struct Bits {
            a: u1,
        }
    );
    let mut bits = Bits(0b0);
    bits.set_a(0b1);
    assert_eq!(bits.a(), 0b1);
}

#[test]
fn two_bits() {
    bitmap!(
        struct Bits {
            a: u1,
            b: u1,
        }
    );
    let mut bits = Bits(0b10);
    bits.set_a(0b1);
    bits.set_b(0b0);
    assert_eq!(*bits, 0b10);
}

#[test]
fn sixty_four_bits_funky_layout() {
    bitmap!(
        struct Bits {
            a: u1,
            b: u7,
            c: u7,
            d: u7,
            e: u7,
            f: u7,
            g: u7,
            h: u7,
            i: u7,
            j: u7,
        }
    );
    let mut bits = Bits(0xFF00FF00FF00FF00);
    bits.set_j(0b0000000).set_i(0b1111111).set_a(0b1);
    assert_eq!(*bits, 0xFF00FF00FF00FF80);
}

#[test]
fn sixty_four_bits_aligned() {
    bitmap!(
        struct Bits {
            a: u32,
            b: u32,
        }
    );
    let mut bits = Bits(0xFF00FF00FF00FF00);
    bits.set_a(0xFFFFFFFF).set_b(0b00000000);
    assert_eq!(*bits, 0xFFFFFFFF00000000);
}

#[test]
fn hundred_and_twenty_eight_bits_funky_layout() {
    bitmap!(
        struct Bits {
            a: u40,
            b: u25,
            c: u31,
            d: u16,
            e: u9,
            f: u7,
        }
    );

    let mut bits = Bits(0xFF00FF00FF00FF00FF00FF00FF00FF00);
    bits.set_a(0xAAAAAAAAAA)
        .set_b(0b1111111111111111111111111)
        .set_c(0b0000000000000000000000000000000)
        .set_d(0x6666)
        .set_e(0b111111111)
        .set_f(0b0000000);

    assert_eq!(*bits, 0xaaaaaaaaaaffffff800000006666ff80);
}

#[test]
fn hundred_and_twenty_eight_bits_aligned() {
    bitmap!(
        struct Bits {
            a: u32,
            b: u32,
            c: u32,
            d: u32,
        }
    );
    let mut bits = Bits(0xFF00FF00FF00FF00FF00FF00FF00FF00);
    bits.set_a(0xFFFFFFFF).set_b(0x00000000).set_c(0x42424242).set_d(0x66666666);
    assert_eq!(*bits, 0xFFFFFFFF000000004242424266666666);
}

macro_rules! test_width {
    ($name:ident, $val:literal) => {
        #[test]
        fn $name() {
            bitmap!(
                struct Bits {
                    field: $name,
                }
            );
            let mut bits = Bits(0);
            bits.set_field($val);
            assert_eq!(bits.field(), $val);
        }
    };
}

include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));
