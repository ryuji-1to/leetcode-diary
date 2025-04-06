pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let (pos, neg) = nums.iter().fold((0, 0), |mut acc, &x| {
        if x > 0 {
            acc.0 += 1;
        } else if x < 0 {
            acc.1 += 1;
        }
        acc
    });
    pos.max(neg)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, maximum_count(vec![-2, -1, -1, 1, 2, 3]));
        assert_eq!(3, maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]));
        assert_eq!(4, maximum_count(vec![5, 20, 66, 1314]));
    }
}
