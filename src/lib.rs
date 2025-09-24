#[cfg(test)]
use macros::bitmap;

#[test]
fn one_bit() {
    bitmap!(
        struct Bits {
            field0: u1,
        }
    );
    let mut bits = Bits(0b0);
    bits.set_field0(0b1);
    assert!(bits.field0() == 0b1);
}

#[test]
fn two_bits() {
    bitmap!(
        struct Bits {
            field0: u1,
            field1: u1,
        }
    );
    let mut bits = Bits(0b10);
    bits.set_field0(0b1);
    bits.set_field1(0b0);
    assert!(bits.0 == 0b01);
}

#[test]
fn sixty_four_bits() {
    bitmap!(
        struct Bits {
            field0: u1,
            field1: u7,
            field2: u7,
            field3: u7,
            field4: u7,
            field5: u7,
            field6: u7,
            field7: u7,
            field8: u7,
            field9: u7,
        }
    );
    let mut bits = Bits(0xFF00FF00FF00FF00);
    bits.set_field9(0b0000000);
    assert!(bits.0 == 0x0100FF00FF00FF00);
}
