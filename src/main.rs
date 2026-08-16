use std::error::Error;

// 試し割り法により因数分解する。
fn factorization(n: i64) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut n_: i64 = n;

    while n_ % 2 == 0 {
        result.push(2);
        n_ /= 2;
    }

    let mut f: i64 = 2;
    while f * f <= n_ {
        if n_ % f == 0 {
            result.push(f);
            n_ /= f;
        } else {
            f += 2;
        }
    }

    if n_ != 1 {
        result.push(n_);
    }

    result
}

// ユークリッドの互除法により最大公約数を求める。
fn gcd(a: i64, b: i64) -> i64 {
    gcd_(std::cmp::max(a, b), std::cmp::min(a, b))
}

fn gcd_(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_(b, a % b)
}

// 積を最大公約数で割り最小公倍数を求める。
fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

// 拡張ユーグリッドの互除法
// ax + by = g (g = gcd(a, b) >= 0) となる (g, x, y) を返す。
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        if a < 0 {
            return (-a, -1, 0);
        } else {
            return (a, 1, 0);
        }
    }
    let (g, xd, yd): (i64, i64, i64) = extended_gcd(b, a % b);
    (g, yd, xd - a / b * yd)
}

// 繰り返し二乗法によるべき乗
fn pow_positive(x: i64, n: i64, p: i64) -> i64 {
    let modulus = p as u128;
    let mut base = (x % p) as u128;
    let mut exp: i64 = n;
    let mut result: u128 = 1 % modulus;

    while exp > 0 {
        if (exp & 1) != 0 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }

    result as i64
}

// 拡張ユーグリッドの互除法によるモジュラ逆元 (x * inv % p = 1)
fn modulus_inverse(x: i64, p: i64) -> Option<i64> {
    let (mut a, mut b) = (x.rem_euclid(p), p);
    let (mut u, mut v) = (1 as i64, 0 as i64);

    while b != 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }

    if a == 1 {
        // %演算子では負の数に対して負の余りが変えるため、rem_euclidで 0<r<p を返す。
        Some(u.rem_euclid(p))
    } else {
        // 逆元が存在しない
        None
    }
}

fn pow(x: i64, n: i64, p: i64) -> Option<i64> {
    if p == 1 {
        return Some(0);
    }

    if n >= 0 {
        Some(pow_positive(x, n, p))
    } else {
        let inv = modulus_inverse(x, p)?;
        Some(pow_positive(inv, n.abs(), p))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_factorization_1() {
        assert_eq!(factorization(12), vec![2, 2, 3]);
    }

    #[test]
    fn test_factorization_2() {
        assert_eq!(factorization(1000000007), vec![1000000007]);
    }

    #[test]
    fn test_gcd_1() {
        assert_eq!(gcd(32, 20), 4);
    }

    #[test]
    fn test_gcd_2() {
        assert_eq!(gcd(13, 17), 1);
    }

    #[test]
    fn test_lcm_1() {
        assert_eq!(lcm(4, 6), 12);
    }

    #[test]
    fn test_lcm_2() {
        assert_eq!(lcm(5, 7), 35);
    }

    #[test]
    fn test_extended_gcd_1() {
        assert_eq!(extended_gcd(11, 4), (1, -1, 3));
    }

    #[test]
    fn test_extended_gcd_2() {
        assert_eq!(extended_gcd(4, 11), (1, 3, -1));
    }

    #[test]
    fn test_extended_gcd_3() {
        assert_eq!(extended_gcd(-3, 5), (1, -2, -1));
    }

    #[test]
    fn test_extended_gcd_4() {
        assert_eq!(extended_gcd(-3, -5), (1, -2, 1));
    }

    #[test]
    fn test_pow_1() {
        assert_eq!(pow(3, 8, 1000000007).unwrap(), 6561);
    }

    #[test]
    fn test_pow_2() {
        assert_eq!(pow(2, 10, 1000000007).unwrap(), 1024);
    }

    #[test]
    fn test_pow_3() {
        assert_eq!(pow(2, -1, 1000000007).unwrap(), 500000004);
        assert_eq!(((10 as i64 * 500000004 as i64) % 1000000007 as i64) as i32, 5 as i32);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    return Ok(());
}
