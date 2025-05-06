#[allow(unused)]
pub fn gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (0, b) => b,
        (a, 0) => a,
        (a, b) if a > b => gcd(a % b, b),
        (a, b) => gcd(a, b % a),
    }
}

#[cfg(test)]
#[test]
fn test_gcd() {
    assert_eq!(gcd(24, 60), 12);
    assert_eq!(gcd(0, 7), 7);
    assert_eq!(gcd(3, 0), 3);
    assert_eq!(gcd(11, 11), 11);
    assert_eq!(gcd(0, 0), 0);
}

#[allow(unused)]
pub fn pow(value: u64, n: u64) -> u64 {
    let mut value = value;
    let mut n = n;
    if n == 0 {
        return 1_u64;
    }
    let mut res = 1;
    while n > 0 {
        if n % 2 != 0 {
            res *= value;
            n -= 1;
        } else {
            value = value * value;
            n >>= 1;
        }
    }
    res
}

#[cfg(test)]
#[test]
fn test_pow() {
    assert_eq!(pow(1, 3), 1);
    assert_eq!(pow(0, 3), 0);
    assert_eq!(pow(2, 20), 1048576);
}

#[allow(unused)]
pub fn pow_mod(mut value: u64, mut n: u64, m: u64) -> u64 {
    if n == 0 {
        return 1_u64;
    }
    let mut res = 1;
    while n > 0 {
        if n % 2 != 0 {
            res = (res % m * value % m) % m;
            n -= 1;
        } else {
            value = (value % m * value % m) % m;
            n >>= 1;
        }
    }
    res
}

#[cfg(test)]
#[test]
fn test_pow_mod() {
    assert_eq!(pow_mod(5, 100, 7), 2);
    assert_eq!(pow_mod(3, 90, 5), 4);
}

#[allow(unused)]
pub fn fibonacci(n: u64) -> u64 {
    let mut n = n;
    let mut a = 1;
    let mut ta;
    let mut b = 1;
    let mut tb;
    let mut c = 1;
    let mut rc = 0;
    let mut tc;
    let mut d = 0;
    let mut rd = 1;
    while n > 0 {
        if n % 2 != 0 {
            tc = rc;
            rc = rc * a + rd * c;
            rd = tc * b + rd * d;
        }
        ta = a;
        tb = b;
        tc = c;
        a = a * a + b * c;
        b = ta * b + b * d;
        c = c * ta + d * c;
        d = tc * tb + d * d;
        n >>= 1;
    }
    rc
}

#[cfg(test)]
#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(24), 46368);
    assert_eq!(fibonacci(36), 14930352);
    assert_eq!(fibonacci(40), 102334155);
}

use rand::Rng;

#[allow(unused)]
pub fn is_simple(value: u64) -> bool {
    if value == 2 {
        return true;
    }
    let mut rng = rand::rng();
    for _ in 0..100 {
        let a = rng.random_range(1..u64::MAX) % (value - 2) + 2;
        if gcd(a, value) != 1 {
            return false;
        }
        if pow_mod(a, value - 1, value) != 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[test]
#[ignore]
fn test_is_simple() {
    fn generate_simple_numbers(max_value: u64) -> Vec<u64> {
        let mut src = vec![true; max_value as usize + 1];
        let mut dst = Vec::new();
        for i in 2..max_value as usize + 1 {
            if src[i] {
                let mut ind = i * i;
                while ind <= max_value as usize {
                    src[ind] = false;
                    ind += i;
                }
                dst.push(i as u64);
            }
        }
        dst
    }

    for value in generate_simple_numbers(1000000) {
        assert!(is_simple(value));
    }
    assert!(!is_simple(10));
    assert!(!is_simple(169));
    assert!(!is_simple(8505));
    assert!(!is_simple(83521));
    assert!(!is_simple(34012224));
    assert!(!is_simple(39916800));
}
