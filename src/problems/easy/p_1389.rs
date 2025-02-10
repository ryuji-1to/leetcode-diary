pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..nums.len() {
        result.insert(index[i] as usize, nums[i]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![0, 4, 1, 3, 2],
            create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1])
        );
        assert_eq!(
            vec![0, 4, 1, 3, 2],
            create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1])
        );
        assert_eq!(
            vec![0, 4, 1, 3, 2],
            create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1])
        );
    }
}
