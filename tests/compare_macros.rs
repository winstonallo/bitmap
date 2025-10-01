use bitmap::{bitmap, bitmap_attr};

bitmap!(
    struct OldWay {
        flag: u1,
        counter: u7,
    }
);

#[bitmap_attr]
struct NewWay {
    flag: u1, 
    counter: u7,
}

#[test]
fn test_both_macros_produce_same_api() {
    let mut old = OldWay(0);
    let mut new = NewWay(0);
    
    old.set_flag(1).set_counter(42);
    new.set_flag(1).set_counter(42);
    
    assert_eq!(old.flag(), new.flag());
    assert_eq!(old.counter(), new.counter());
    assert_eq!(*old, *new);
    
    println!("Both macros produce identical results!");
}
