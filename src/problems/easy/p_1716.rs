pub fn total_money(n: i32) -> i32 {
    let weeks = n / 7;
    let rest = n % 7;
    let first_week_sum = (7 * 8) / 2;
    first_week_sum * weeks + 7 * weeks * (weeks - 1) / 2 + rest * (rest + 1) / 2 + rest * weeks
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(10, total_money(4));
        assert_eq!(37, total_money(10));
        assert_eq!(96, total_money(20));
    }
}
