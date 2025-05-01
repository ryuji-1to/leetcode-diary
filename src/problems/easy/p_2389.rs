pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    queries
        .iter()
        .map(|&x| {
            let mut result = (0, 0);
            for &v in &nums {
                result.1 += v;
                if result.1 > x {
                    return result.0;
                }
                result.0 += 1;
            }
            result.0
            // let mut sum = 0;
            // nums.iter()
            //     .position(|&num| {
            //         sum += num;
            //         sum > x
            //     })
            //     .unwrap_or(nums.len()) as i32
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![2, 3, 4],
            answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21])
        );
        assert_eq!(vec![0], answer_queries(vec![2, 3, 4, 5], vec![1]));
    }
}
