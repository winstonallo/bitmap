use macros::bitmap;

fn main() {
    bitmap!(
        struct Bits {
            field0: u8,
        }
    );
}
