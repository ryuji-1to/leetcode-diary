pub fn number_of_matches(mut n: i32) -> i32 {
    let mut result = 0;
    while n != 1 {
        if n % 2 == 1 {
            result += (n - 1) / 2;
            n = (n - 1) / 2 + 1;
        } else {
            n = n / 2;
            result += n;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(6, number_of_matches(7));
        assert_eq!(13, number_of_matches(14));
    }
}
