pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::from([0, 0]);
    // for i in 0..nums1.len() {
    //     let target = nums1[i];
    //     if nums2.contains(&target) {
    //         result[0] += 1;
    //     }
    // }
    // for i in 0..nums2.len() {
    //     let target = nums2[i];
    //     if nums1.contains(&target) {
    //         result[1] += 1;
    //     }
    // }
    for i in 0..nums1.len().max(nums2.len()) {
        let t1 = nums1.get(i);
        let t2 = nums2.get(i);
        match t1 {
            Some(v) => {
                if nums2.contains(v) {
                    result[0] += 1;
                }
            }
            None => {}
        }
        match t2 {
            Some(v) => {
                if nums1.contains(v) {
                    result[1] += 1;
                }
            }
            None => {}
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![2, 1],
            find_intersection_values(vec![2, 3, 2], vec![1, 2])
        );
        assert_eq!(
            vec![3, 4],
            find_intersection_values(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6])
        );
        assert_eq!(
            vec![0, 0],
            find_intersection_values(vec![3, 4, 2, 3], vec![1, 5])
        );
        assert_eq!(
            vec![4, 2],
            find_intersection_values(
                vec![24, 28, 7, 27, 7, 27, 9, 24, 9, 10],
                vec![12, 29, 9, 7, 4]
            )
        );
    }
}
