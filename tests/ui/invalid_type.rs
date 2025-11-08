use macros::bitstruct;

fn main() {
    #[bitstruct]
    struct Bits {
        field0: u129,
    }
}
