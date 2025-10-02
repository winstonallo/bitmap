use macros::bitmap;

fn main() {
    #[bitmap]
    struct Bits {
        field0: u128,
        field1: u1,
    }
}
