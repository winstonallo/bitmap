use std::{fs::File, io::Write, path::Path};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_tests.rs");
    let mut f = File::create(&dest_path).unwrap();

    for n in 1..=128 {
        let max_val = if n != 128 {
            (1u128 << n) - 1
        } else {
            340282366920938463463374607431768211455
        };
        writeln!(f, "test_width!(u{n}, {max_val});").unwrap();
    }
}
