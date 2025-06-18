pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
    stones.sort_unstable_by(|a, b| b.cmp(a));
    while stones.len() > 1 {
        println!("{:?}", stones);
        let a1 = stones[0];
        let a2 = stones[1];
        stones = stones[2..].to_vec();
        if a1 != a2 {
            stones.push(a1.abs_diff(a2) as i32);
            stones.sort_unstable_by(|a, b| b.cmp(a));
        }
    }
    *stones.get(0).unwrap_or(&0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
        assert_eq!(1, last_stone_weight(vec![1]));
        assert_eq!(3, last_stone_weight(vec![7, 6, 7, 6, 9]));
        assert_eq!(2, last_stone_weight(vec![9, 10, 1, 7, 3]));
    }
}
