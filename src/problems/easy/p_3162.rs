pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    // let mut result = Vec::new();
    let mut result = 0;
    for i in 0..nums1.len() {
        for j in 0..nums2.len() {
            if nums1[i] % (nums2[j] * k) == 0 {
                result += 1;
                // result.push((i, j));
            }
        }
    }
    // result.len() as i32
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5, number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1));
        assert_eq!(2, number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3));
    }
}
