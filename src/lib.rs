use macros::bitmap;

#[test]
fn it_works() {
    bitmap!(
        struct Bits {
            field1: u1,
        }
    );
    let mut bits = Bits(0);
    bits.set_field1(true);
    assert!(bits.field1() == 1)
}
