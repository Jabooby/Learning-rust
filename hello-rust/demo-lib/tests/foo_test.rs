use demo_lib::add;


#[test]
fn foo() {
    assert!(false);
}
#[test]
fn foo1() {
    assert!(true);
}
#[test]
fn foo2() {
    assert!(true);
}
#[test]
fn foo3() {
    assert!(false);
}

/// THIS IS GREAT DOC WOWIE!
#[test]
fn foo_test()
{
    assert_eq!(4, add(2,2));
}