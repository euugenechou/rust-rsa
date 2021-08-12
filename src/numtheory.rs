#![allow(dead_code)]

use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};

fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let mut x = a.clone();
    let mut y = b.clone();

    while y != Zero::zero() {
        let t = y.clone();
        y = x % y;
        x = t;
    }

    x
}

fn inverse(a: &BigInt, n: &BigInt) -> Option<BigInt> {
    let mut r = n.clone();
    let mut rp = a.clone();
    let mut t = BigInt::from(0);
    let mut tp = BigInt::from(1);

    while rp != Zero::zero() {
        let q = &r / &rp;

        let tmp = r.clone();
        r = rp.clone();
        rp = tmp - &q * &rp;

        let tmp = t.clone();
        t = tp.clone();
        tp = tmp - &q * &tp;
    }

    if r > One::one() {
        return None;
    }
    if t < Zero::zero() {
        t += n;
    }

    Some(t)
}

fn odd(n: &BigInt) -> bool {
    n & BigInt::from(1) == BigInt::from(1)
}

fn even(n: &BigInt) -> bool {
    n & BigInt::from(1) == BigInt::from(0)
}

fn powermod(a: &BigInt, d: &BigInt, n: &BigInt) -> BigInt {
    let mut p = a.clone();
    let mut t = d.clone();
    let mut v = BigInt::from(1);

    while t > Zero::zero() {
        if odd(&t) {
            v = (&v * &p) % n;
        }
        p = (&p * &p) % n;
        t >>= 1;
    }

    v
}

fn isprime(n: &BigInt, k: u64) -> bool {
    if n < &BigInt::from(2) {
        return false; // 0 and 1 aren't prime.
    }
    if n < &BigInt::from(4) {
        return true; // 2 and 3 are prime.
    }
    if even(n) {
        return false; // Evens after 2 aren't prime.
    }

    // Write n - 1 = r2^s such that r is odd.
    let mut s = BigInt::from(0);
    let mut r = n - 1;
    while even(&r) {
        s += 1;
        r >>= 1;
    }

    let mut rng = rand::thread_rng();

    for _ in 1..=k {
        let a = rng.gen_bigint_range(&BigInt::from(2), &(n - 1));
        let mut y = powermod(&a, &r, n);

        if y != One::one() && y != n - 1 {
            let mut j = BigInt::from(1);
            while j <= &s - 1 && y != n - 1 {
                y = powermod(&y, &BigInt::from(2), n);
                if y == One::one() {
                    return false;
                }
                j += 1;
            }

            if y != n - 1 {
                return false;
            }
        }
    }

    true
}

fn makeprime(bits: u64, iters: u64) -> BigInt {
    let mut rng = rand::thread_rng();
    let mut prime = rng.gen_bigint(bits);

    while !isprime(&prime, iters) {
        prime = rng.gen_bigint(bits);
    }

    prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let tests = [
            (30, 18, 6),
            (7, 7, 7),
            (20, 100, 20),
            (624129, 2061517, 18913),
            (37, 600, 1),
        ];

        for (a, b, divisor) in tests {
            let a = BigInt::from(a);
            let b = BigInt::from(b);
            let divisor = BigInt::from(divisor);
            assert_eq!(gcd(&a, &b), divisor);
        }
    }

    #[test]
    fn test_inverse() {
        let tests = [
            (5, 13, Some(BigInt::from(8))),
            (1, 2, Some(BigInt::from(1))),
            (3, 6, None),
            (7, 87, Some(BigInt::from(25))),
            (25, 87, Some(BigInt::from(7))),
            (2, 91, Some(BigInt::from(46))),
            (13, 91, None),
            (19, 1212393831, Some(BigInt::from(701912218))),
            (31, 73714876143i64, Some(BigInt::from(45180085378i64))),
            (3, 73714876143i64, None),
        ];

        for (a, n, inv) in tests {
            let a = BigInt::from(a);
            let n = BigInt::from(n);
            assert_eq!(inverse(&a, &n), inv);
        }
    }

    #[test]
    fn test_powermod() {
        let tests = [
            (2, 8, 255, 1),
            (2, 8, 256, 0),
            (2, 8, 257, 256),
            (3, 7, 10000, 2187),
            (2, 2046, 2047, 1),
            (123, 456, 789, 699),
            (3, 1000, 18446744073709551615i128, 12311760789144243126i128),
            (86400, 22157322, 48519018006822, 40149207423504),
            (
                8675309,
                100018327824i64,
                8621993634251008000,
                3858055581225668161,
            ),
            (
                325284989554104320i64,
                1508436685178379520,
                8582294829391072256,
                6354230931838838784,
            ),
        ];

        for (base, exponent, modulus, result) in tests {
            let a = BigInt::from(base);
            let d = BigInt::from(exponent);
            let n = BigInt::from(modulus);
            let r = BigInt::from(result);
            assert_eq!(powermod(&a, &d, &n), r);
        }
    }

    #[test]
    fn test_isprime() {
        let tests = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (41041, false),
            (46657, false),
            (52633, false),
            (62745, false),
            (63973, false),
            (252601, false),
            (3057601, false),
            (104717, true),
            (577757, true),
            (101089, true),
            (280001, true),
            (100000004677i64, true),
            (100000004678i64, false),
        ];

        for (n, primality) in tests {
            let n = BigInt::from(n);
            assert_eq!(isprime(&n, 50), primality);
        }
    }
}
