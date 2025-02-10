pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    let mut result = nums.clone();
    for i in 0..k {
        let minimum = result.iter().min().unwrap();
        let first = result.iter().position(|x| x == minimum).unwrap();
        result[first] *= multiplier;
    }
    result
}
// ↓ ChatGPT's solution
// pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
//     for _ in 0..k {
//         if let Some((idx, _)) = nums.iter().enumerate().min_by_key(|&(_, &val)| val) {
//             nums[idx] *= multiplier;
//         }
//     }
//     nums
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![8, 4, 6, 5, 6],
            get_final_state(vec![2, 1, 3, 5, 6], 5, 2)
        );
        assert_eq!(vec![16, 8], get_final_state(vec![1, 2], 3, 4));
    }
}
