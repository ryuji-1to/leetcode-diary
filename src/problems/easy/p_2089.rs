pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    nums.sort_unstable();
    for i in 0..nums.len() {
        if nums[i] == target {
            result.push(i as i32);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![1, 2], target_indices(vec![1, 2, 5, 2, 3], 2));
        assert_eq!(vec![3], target_indices(vec![1, 2, 5, 2, 3], 3));
        assert_eq!(vec![4], target_indices(vec![1, 2, 5, 2, 3], 5));
    }
}
