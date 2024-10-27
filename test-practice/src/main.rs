fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3); // This test will pass
}

#[test]
fn test_add2() {
    assert_eq!(add(1, 3), 5); // This test will fail
}


fn main() {
    println!("{}", add(1, 2));
}
