pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
    prices.sort_unstable();
    let total = prices[0] + prices[1];
    if total <= money {
        money - total
    } else {
        money
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, buy_choco(vec![1, 2, 2], 3));
        assert_eq!(3, buy_choco(vec![3, 2, 3], 3));
    }
}
