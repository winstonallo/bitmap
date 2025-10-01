use bitmap::bitmap_attr;

#[bitmap_attr]
struct TestBits {
    flag: u1,
    counter: u7,
}

#[test]
fn test_attribute_macro_creates_correct_api() {
    let mut bits = TestBits(0);
    
    bits.set_flag(1);
    bits.set_counter(42);
    
    assert_eq!(bits.flag(), 1);
    assert_eq!(bits.counter(), 42);
    
    assert_eq!(*bits, 0b10101010); 
    
    println!("Attribute macro creates correct bitmap API!");
}

#[test] 
fn test_attribute_macro_deref_and_into() {
    let bits = TestBits(0xFF);
    let raw_value: u8 = bits.into();
    assert_eq!(raw_value, 0xFF);
    assert_eq!(*bits, 0xFF);
}
