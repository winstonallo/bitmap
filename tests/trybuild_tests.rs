#[test]
fn bitmap_too_large() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/bitmap_too_large.rs");
}

#[test]
fn invalid_type() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/invalid_type.rs");
}
