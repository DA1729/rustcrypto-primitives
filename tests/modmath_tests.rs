use rustcrypto_primitives::modmath::*;

#[test]
fn test_mod_add() {
    assert_eq!(mod_add(3, 5, 7), 1);
}

#[test]
fn test_mod_sub() {
    assert_eq!(mod_sub(3, 5, 7), 5);
}

#[test]
fn test_mod_mul() {
    assert_eq!(mod_mul(4, 5, 7), 6);
}

#[test]
fn test_mod_exp() {
    assert_eq!(mod_exp(2, 10, 1000), 24);
}

#[test]
fn test_mod_inv() {
    assert_eq!(mod_inv(3, 11), Some(4));
    assert_eq!(mod_inv(10, 17), Some(12));
    assert_eq!(mod_inv(2, 4), None); // no inverse
}

