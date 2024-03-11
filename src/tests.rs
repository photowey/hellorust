use super::*;

#[test]
fn test_say_hi() {
    let words = say_hi();

    assert_eq!(words, "say hello to: sharkchili");
}

// $ cargo test -- --show-output

#[test]
fn test_add_two() {
    assert_eq!(4, add_two(2));
    assert_eq!(4, inner_add_two(2, 2));
}

#[test]
fn test_capitalize_first() {
    let x = capitalize_first("hello");
    assert_eq!("Hello", x);
}

#[test]
fn test_env_run() {
    env::run();
}