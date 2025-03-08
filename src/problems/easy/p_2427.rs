pub fn commmon_factors(a: i32, b: i32) -> i32 {
    (1..=(a.min(b))).fold(0, |acc, x| {
        if a % x == 0 && b % x == 0 {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, commmon_factors(12, 6));
        assert_eq!(2, commmon_factors(25, 30));
    }
}
