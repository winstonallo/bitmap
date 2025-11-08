use macros::bitstruct;

fn main() {
    #[bitstruct]
    struct Bits {
        field0: u128,
        field1: u1,
    }
}
