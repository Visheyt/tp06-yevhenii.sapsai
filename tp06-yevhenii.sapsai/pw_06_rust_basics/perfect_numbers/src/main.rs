use divisor_sum::divisor_sum;

fn main() {
    let mut n = 1;
    while n <= 100000 {
        if is_perfect_number(n) {
            println!("{}", n);
        }
        n += 1
    }
}

fn is_perfect_number(n: u64) -> bool {
    let mut f: bool = false;
    if divisor_sum(n) == n {
        f = true;
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_is_perfect_number() {
        assert_eq!(is_perfect_number(6), true);
        assert_eq!(is_perfect_number(28), true);
        assert_eq!(is_perfect_number(496), true);
        assert_eq!(is_perfect_number(8128), true);
    }
}
