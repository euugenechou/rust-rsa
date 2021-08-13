use num_bigint::{BigInt, BigUint, RandBigInt, Sign};
use num_traits::{One, Zero};

pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    let mut x = a.clone();
    let mut y = b.clone();

    while y != BigUint::zero() {
        let t = y.clone();
        y = x % y;
        x = t;
    }

    x
}

pub fn inverse(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    let mut r = BigInt::from_biguint(Sign::Plus, n.clone());
    let mut rp = BigInt::from_biguint(Sign::Plus, a.clone());
    let mut t = BigInt::zero();
    let mut tp = BigInt::one();

    while rp != BigInt::zero() {
        let q = &r / &rp;

        let tmp = r.clone();
        r = rp.clone();
        rp = tmp - &q * &rp;

        let tmp = t.clone();
        t = tp.clone();
        tp = tmp - &q * &tp;
    }

    if r > BigInt::one() {
        return None;
    }
    if t < BigInt::zero() {
        t += BigInt::from_biguint(Sign::Plus, n.clone());
    }

    Some(t.magnitude().clone())
}

fn odd(n: &BigUint) -> bool {
    n & BigUint::one() == BigUint::one()
}

fn even(n: &BigUint) -> bool {
    n & BigUint::one() == BigUint::zero()
}

pub fn powermod(a: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    let mut p = a.clone();
    let mut t = d.clone();
    let mut v = BigUint::one();

    while t > BigUint::zero() {
        if odd(&t) {
            v = (&v * &p) % n;
        }
        p = (&p * &p) % n;
        t >>= 1;
    }

    v
}

fn isprime(n: &BigUint) -> bool {
    if n < &BigUint::from(2u8) {
        return false; // 0 and 1 aren't prime.
    }
    if n < &BigUint::from(4u8) {
        return true; // 2 and 3 are prime.
    }
    if even(n) {
        return false; // Evens after 2 aren't prime.
    }

    // Write n - 1 = r2^s such that r is odd.
    let mut s = BigUint::zero();
    let mut r = n - 1u8;
    while even(&r) {
        s += 1u8;
        r >>= 1u8;
    }

    let mut rng = rand::thread_rng();

    // 50 Miller-Rabin iterations.
    for _ in 1..=50 {
        let a = rng.gen_biguint_range(&BigUint::from(2u8), &(n - 1u8));
        let mut y = powermod(&a, &r, n);

        if y != BigUint::one() && y != n - 1u8 {
            let mut j = BigUint::from(1u8);
            while j < s && y != n - 1u8 {
                y = powermod(&y, &BigUint::from(2u8), n);
                if y == BigUint::one() {
                    return false;
                }
                j += 1u8;
            }

            if y != n - 1u8 {
                return false;
            }
        }
    }

    true
}

pub fn makeprime(bits: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut prime = rng.gen_biguint(bits);

    while !isprime(&prime) {
        prime = rng.gen_biguint(bits);
    }

    prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let tests: [(u128, u128, u128); 5] = [
            (30, 18, 6),
            (7, 7, 7),
            (20, 100, 20),
            (624129, 2061517, 18913),
            (37, 600, 1),
        ];

        for (a, b, divisor) in &tests {
            let a = BigUint::from(*a);
            let b = BigUint::from(*b);
            let divisor = BigUint::from(*divisor);
            assert_eq!(gcd(&a, &b), divisor);
        }
    }

    #[test]
    fn test_inverse() {
        let tests: [(u128, u128, Option<BigUint>); 10] = [
            (5, 13, Some(BigUint::from(8u8))),
            (1, 2, Some(BigUint::from(1u8))),
            (3, 6, None),
            (7, 87, Some(BigUint::from(25u8))),
            (25, 87, Some(BigUint::from(7u8))),
            (2, 91, Some(BigUint::from(46u8))),
            (13, 91, None),
            (19, 1212393831, Some(BigUint::from(701912218u128))),
            (31, 73714876143, Some(BigUint::from(45180085378u128))),
            (3, 73714876143, None),
        ];

        for (a, n, inv) in &tests {
            let a = BigUint::from(*a);
            let n = BigUint::from(*n);
            assert_eq!(inverse(&a, &n), *inv);
        }
    }

    #[test]
    fn test_powermod() {
        let tests: [(u128, u128, u128, u128); 10] = [
            (2, 8, 255, 1),
            (2, 8, 256, 0),
            (2, 8, 257, 256),
            (3, 7, 10000, 2187),
            (2, 2046, 2047, 1),
            (123, 456, 789, 699),
            (3, 1000, 18446744073709551615, 12311760789144243126),
            (86400, 22157322, 48519018006822, 40149207423504),
            (
                8675309,
                100018327824,
                8621993634251008000,
                3858055581225668161,
            ),
            (
                325284989554104320,
                1508436685178379520,
                8582294829391072256,
                6354230931838838784,
            ),
        ];

        for (base, exponent, modulus, result) in &tests {
            let a = BigUint::from(*base);
            let d = BigUint::from(*exponent);
            let n = BigUint::from(*modulus);
            let r = BigUint::from(*result);
            assert_eq!(powermod(&a, &d, &n), r);
        }
    }

    #[test]
    fn test_isprime() {
        let tests: [(u128, bool); 18] = [
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
            (100000004677, true),
            (100000004678, false),
        ];

        for (n, primality) in &tests {
            let n = BigUint::from(*n);
            assert_eq!(isprime(&n), *primality);
        }
    }
}
