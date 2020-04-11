fn main() {
    // TODO
    let mut n = 0;
    while n <= 90 {
        println!("fibo({}) = {}", n, fibo(n));
        n += 1
    }
}
////Recursion method - very slow after number 30////////
/*fn fibo(n: u64) -> u64
{
    // TODO
    if n<=2
    {
        1
    }
    else
    {
        fibo(n-1)+fibo(n-2)
    }
   // unimplemented!()
}*/

fn fibo(n: i64) -> i64 {
    let mut a = 1;
    let mut i=0;
    let mut b = 1;
    /*for _ in 0..n {
        let tmp = a;
        a = b;
        b = a + tmp;
    }*/
    while i<=n
    {
        if i==0
        {
            b=0;
        }
        else {
            let tmp = a;
            a = b;
            b = a + tmp;
        }
        i+=1
    }

    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibo() {
        assert_eq!(fibo(0), 0);
        assert_eq!(fibo(1), 1);
        assert_eq!(fibo(2), 1);
        assert_eq!(fibo(3), 2);
        assert_eq!(fibo(4), 3);
        assert_eq!(fibo(5), 5);
        assert_eq!(fibo(6), 8);
    }
}
