fn main() {
    let mut n = 0;
    while n < 64 {
        println!("power_of_two({}) = {}", n, power_of_two(n));
        n += 1
    }
}

fn power_of_two(n: u8) -> u64 {
    let one = 1;
    one << n
    /*  if n==0
    {
        1
    }
    else {
     two<<n-1
    }*/
    // unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_power_of_two() {
        assert_eq!(power_of_two(0), 1);
        assert_eq!(power_of_two(2), 4);
        assert_eq!(power_of_two(21), 2097152);
        assert_eq!(power_of_two(44), 17592186044416);
        assert_eq!(power_of_two(56), 72057594037927936);
        assert_eq!(power_of_two(63), 9223372036854775808);
    }
}
