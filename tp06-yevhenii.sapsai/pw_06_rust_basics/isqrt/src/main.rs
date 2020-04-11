fn main() {
    let mut n = 0;
    while n <= 200 {
        println!("isqrt({}) = {}", n, istqrt(n));
        istqrt(n);
        n += 8
    }
}

fn istqrt(n: u64) -> u64 {
    let mut r = n;
    while (r * r) > n {
        r = r + n / r;
        r = r / 2;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_istqrt() {
        assert_eq!(istqrt(0), 0);
        assert_eq!(istqrt(96), 9);
        assert_eq!(istqrt(192), 13);
    }
}
