# Rust Crypto Learning Project
[![Build Status](https://img.shields.io/github/actions/workflow/status/DA1729/rustcrypto-primitives/ci.yml)](https://github.com/DA1729/rustcrypto-primitives/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

> **Learning Project**  
> This is an experimental implementation for educational purposes only.  
> Not suitable for real-world cryptographic use.

## Current Progress

### Implemented Modules:
- `modmath`: Modular arithmetic operations
- `primes`: Prime number generation/verification
- `consttime`: Constant-time utilities

## Basic Usage

```toml
[dependencies]
rustcrypto-primitives = { git = "https://github.com/DA1729/rustcrypto-primitives" }
```

```
use rustcrypto_primitives::{primes::is_prime, modmath::mod_exp};

let p = 53; // Test prime
assert!(is_prime(p, 5));
assert_eq!(mod_exp(2, 10, 1000), 24);
```

## Coming Soon
- Symmetric encryption (AES)
- Hash functions (SHA-256)
- Public-key cryptography

*I'm actively expanding this project as I learn. Check back for updates!*

## Key Features
- **Minimalist** - Only essential info
- **Clear Disclaimer** - Emphasizes learning purpose
- **Expandable Structure** - Easy to add new sections
- **Basic Usage Example** - Just enough to test
- **Forward-Looking** - Mentions planned work


