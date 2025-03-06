pub fn max_coins(mut piles: Vec<i32>) -> i32 {
    piles.sort_unstable_by(|a, b| b.cmp(a));
    let mut result = 0;
    let op_count = piles.len() / 3;
    for i in 0..op_count {
        result += piles[i * 2 + 1];
    }
    result
}
/*
pub fn max_coins(mut piles: Vec<i32>) -> i32 {
    let n = piles.len() / 3;
    piles.sort_unstable();
    piles.into_iter().skip(n).step_by(2).sum()
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(9, max_coins(vec![2, 4, 1, 2, 7, 8]));
        assert_eq!(4, max_coins(vec![2, 4, 5]));
        assert_eq!(18, max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]));
    }
}
