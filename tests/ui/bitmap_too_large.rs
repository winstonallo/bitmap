use macros::bitmap;

fn main() {
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
            field10: u7,
        }
    );
}
