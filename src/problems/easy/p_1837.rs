pub fn sum_base(mut n: i32, k: i32) -> i32 {
    let mut result = 0;
    // あまりを足して、商を n にする
    while n > 0 {
        result += n % k;
        n /= k;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(9, sum_base(34, 6));
        assert_eq!(1, sum_base(10, 10));
    }
}
