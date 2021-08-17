use rug::{integer::IsPrime, rand::RandState, Integer};

pub fn gcd(a: &Integer, b: &Integer) -> Integer {
    let a = Integer::from(a);
    let b = Integer::from(b);
    a.gcd(&b)
}

pub fn inverse(a: &Integer, n: &Integer) -> Option<Integer> {
    let a = Integer::from(a);
    let n = Integer::from(n);
    match a.invert(&n) {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

pub fn powermod(a: &Integer, d: &Integer, n: &Integer) -> Integer {
    let a = Integer::from(a);
    let d = Integer::from(d);
    let n = Integer::from(n);
    a.pow_mod(&d, &n).unwrap()
}

fn isprime(n: &Integer) -> bool {
    match n.is_probably_prime(30) {
        IsPrime::Yes => true,
        IsPrime::Probably => true,
        IsPrime::No => false,
    }
}

pub fn makeprime(bits: u32) -> Integer {
    let mut state = RandState::new();
    let mut prime = Integer::from(Integer::random_bits(bits, &mut state));

    while !isprime(&prime) {
        prime = Integer::from(Integer::random_bits(bits, &mut state));
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
            let a = Integer::from(*a);
            let b = Integer::from(*b);
            let divisor = Integer::from(*divisor);
            assert_eq!(gcd(&a, &b), divisor);
        }
    }

    #[test]
    fn test_inverse() {
        let tests: [(u128, u128, Option<Integer>); 10] = [
            (5, 13, Some(Integer::from(8u8))),
            (1, 2, Some(Integer::from(1u8))),
            (3, 6, None),
            (7, 87, Some(Integer::from(25u8))),
            (25, 87, Some(Integer::from(7u8))),
            (2, 91, Some(Integer::from(46u8))),
            (13, 91, None),
            (19, 1212393831, Some(Integer::from(701912218u128))),
            (31, 73714876143, Some(Integer::from(45180085378u128))),
            (3, 73714876143, None),
        ];

        for (a, n, inv) in &tests {
            let a = Integer::from(*a);
            let n = Integer::from(*n);
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
            let a = Integer::from(*base);
            let d = Integer::from(*exponent);
            let n = Integer::from(*modulus);
            let r = Integer::from(*result);
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
            let n = Integer::from(*n);
            assert_eq!(isprime(&n), *primality);
        }
    }
}
