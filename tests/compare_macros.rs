// tests/compare_macros.rs
use bitmap::{bitmap, bitmap_attr};

// Function-like macro (original)
bitmap!(
    struct OldWay {
        flag: u1,
        counter: u7,
    }
);

// Attribute macro (new)
#[bitmap_attr]
struct NewWay {
    flag: u1, 
    counter: u7,
}

#[test]
fn test_both_macros_produce_same_api() {
    let mut old = OldWay(0);
    let mut new = NewWay(0);
    
    // Both should have the same methods
    old.set_flag(1).set_counter(42);
    new.set_flag(1).set_counter(42);
    
    // Both should produce the same results
    assert_eq!(old.flag(), new.flag());
    assert_eq!(old.counter(), new.counter());
    assert_eq!(*old, *new);
    
    println!("Both macros produce identical results!");
}