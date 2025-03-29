pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..nums1.len() {
        if nums2.contains(&nums1[i]) && !result.contains(&nums1[i]) {
            result.push(nums1[i]);
        }
    }
    result
    /*
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();
        set1.intersection(&set2).cloned().collect()
    */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(vec![2], intersection(vec![1, 2, 2, 1], vec![2, 2]));
        // assert_eq!(vec![9, 4], intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]));
    }
}
