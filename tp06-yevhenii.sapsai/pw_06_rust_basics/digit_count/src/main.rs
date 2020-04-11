fn main() {
    let mut n = 0;
    let mut x = 1;
    let one: u64 = 1;
    while n < 65 {
        if n > 1 {
            println!("digit_count({}) = {}", one << x, digit_count(one << x));
            n += 1;
            x += 1;
        } else {
            println!("digit_count({}) = {}", n, digit_count(n));
            n += 1
        }
    }
}

fn digit_count(mut n: u64) -> u8 {
    // TODO

    let mut count;
    if n == 0 {
        count = 1
    } else {
        count = 0
    }
    while n != 0 {
        count += 1;
        n /= 10;
    }
    count
    //  todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_digit_count() {
        assert_eq!(digit_count(0), 1);
        assert_eq!(digit_count(16), 2);
        assert_eq!(digit_count(4096), 4);
        assert_eq!(digit_count(8192), 4);
        assert_eq!(digit_count(33554432), 8);
    }
}
