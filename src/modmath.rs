pub fn mod_add(a: u64, b: u64, m: u64) -> u64 {
    (a % m + b % m) % m
}

pub fn mod_sub(a: u64, b: u64, m: u64) -> u64 {
    ((a % m + m) - (b % m)) % m
}

pub fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

// modular exponentiation
pub fn mod_exp(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    
    base %= m;

    while exp > 0{
        if exp % 2 == 1{
            result = mod_mul(result, base, m);
        }

        base = mod_mul(base, base, m);
        exp /= 2;
    }

    result
}


// mod inverse
pub fn mod_inv(a: i64, m: i64) -> Option<i64> {
    let (gcd, x, _) = extended_gcd(a, m);

    if gcd != 1 {
        return None;            // case of no inverse
    }

    Some((x % m + m) % m)
}


// extended euclidean algorithm
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else{
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a/b) * y1)
    }
}
