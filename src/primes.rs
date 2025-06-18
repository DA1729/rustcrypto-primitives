use rand::Rng;
use crate::modmath::mod_exp;

pub fn is_prime(n: u64, k: u32) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 || n == 3 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut r = 0;

    while d % 2 == 0{
        d /= 2;
        r += 1;
    }

    let mut rng = rand::thread_rng();

    'witness: for _ in 0..k {
        let a = rng.gen_range(2..n-2);
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n-1 {
            continue;
        }

        for _ in 0..r-1{
            x = mod_exp(x, 2, n);
            if x == n-1 {
                continue 'witness;
            }
        }
        return false;
    }

    true
}

pub fn gen_prime(bits: usize) -> u64 {
    
    let mut rng = rand::thread_rng();

    loop {
        let candidate = rng.r#gen::<u64>() | 1;
        if candidate.count_ones() >= bits as u32 && is_prime(candidate, 5) {
            return candidate;
        }
    }
}
