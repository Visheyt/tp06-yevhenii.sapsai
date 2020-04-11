fn main() {
    let mut n = 0;
    while n <= 20 {
        println!("facto({}) = {}", n, facto(n));
        n += 1;
    }
}

fn facto(n: u64) -> u64 {
    if n < 1 {
        1
    } else {
        n * facto(n - 1)
    }

    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facto() {
        assert_eq!(facto(0), 1);
        assert_eq!(facto(1), 1);
        assert_eq!(facto(2), 2);
        assert_eq!(facto(8), 40_320);
        assert_eq!(facto(12), 479_001_600);
        assert_eq!(facto(17), 355_687_428_096_000);
        assert_eq!(facto(20), 2_432_902_008_176_640_000);
    }
}
