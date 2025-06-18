use rustcrypto_primitives::primes::{is_prime, gen_prime};

#[test]
fn test_known_primes() {
    let known_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 97];
    for &p in &known_primes {
        assert!(is_prime(p, 10), "Failed on known prime: {}", p);
    }
}

#[test]
fn test_known_composites() {
    let known_composites = [1, 4, 6, 8, 9, 10, 12, 15, 21, 25, 100, 200];
    for &n in &known_composites {
        assert!(!is_prime(n, 10), "Incorrectly identified as prime: {}", n);
    }
}

#[test]
fn test_gen_prime_basic() {
    let p = gen_prime(16); // generate ~16-bit prime
    assert!(is_prime(p, 10), "Generated number is not prime: {}", p);
    assert!(p.leading_zeros() <= 48, "Prime is too small: {}", p);
}

#[test]
fn test_multiple_gen_primes() {
    for _ in 0..5 {
        let p = gen_prime(32);
        assert!(is_prime(p, 10), "Failed: gen_prime gave composite {}", p);
        assert!(p.leading_zeros() <= 32, "Failed: gen_prime gave too small value {}", p);
    }
}

