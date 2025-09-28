pub use macros::bitmap;

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
    assert_eq!(bits.0, 0b01);
}

#[test]
fn sixty_four_bits() {
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
    assert_eq!(bits.0, 0x01FCFF00FF00FF01);
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

test_width!(u1, 1);
test_width!(u2, 3);
test_width!(u3, 7);
test_width!(u4, 15);
test_width!(u5, 31);
test_width!(u6, 63);
test_width!(u7, 127);
