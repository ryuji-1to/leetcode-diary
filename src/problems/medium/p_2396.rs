pub fn is_strictly_palindromic(n: i32) -> bool {
    (2..=n - 2).all(|base| {
        let mut result = Vec::new();
        let mut n = n;
        while n != 0 {
            result.push(n % base);
            n /= base;
        }
        result.iter().eq(result.iter().rev())
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(false, is_strictly_palindromic(9));
        assert_eq!(false, is_strictly_palindromic(4));
    }
}
