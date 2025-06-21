pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
    while nums.len() > 1 {
        nums = nums
            .chunks(2)
            .enumerate()
            .map(|(i, xs)| {
                if i % 2 == 0 {
                    *xs.iter().min().unwrap()
                } else {
                    *xs.iter().max().unwrap()
                }
            })
            .collect::<Vec<_>>();
    }
    nums[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]));
        assert_eq!(3, min_max_game(vec![3]));
    }
}
