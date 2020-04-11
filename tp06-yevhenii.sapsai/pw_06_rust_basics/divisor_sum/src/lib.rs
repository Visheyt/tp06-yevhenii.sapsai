pub fn divisor_sum(n: u64) -> u64 {
    let mut z = 1;
    let mut res = 1;
    let mut result = 0;
    if n == 0 || n == 1 {
        res = n;
    }
    while res <= n {
        z += 1;
        res = z * z;
    }
    result = z - 1;

    let mut j: u64 = 2;
    let mut result2: u64 = 0;
    while j <= result {
        if n % j == 0 {
            if j == (n / j) {
                result2 += j;
            } else {
                result2 += j + n / j;
            }
        }
        j += 1;
    }

    if n > 1 {
        result2 = result2 + 1;
    }
    result2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_divisor_sum() {
        assert_eq!(divisor_sum(1), 0);
        assert_eq!(divisor_sum(3), 1);
        assert_eq!(divisor_sum(6), 6);
        assert_eq!(divisor_sum(9), 4);
        assert_eq!(divisor_sum(11), 1);
        assert_eq!(divisor_sum(28), 28);
        assert_eq!(divisor_sum(30), 42);
        assert_eq!(divisor_sum(496), 496);
    }
}
