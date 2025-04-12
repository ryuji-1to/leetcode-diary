pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    if k > *nums.first().unwrap() {
        return -1;
    }
    let mut result = 0;
    let mut seen = std::collections::HashSet::new();
    for n in nums {
        let r = seen.insert(n);
        if r && n > k {
            result += 1;
        }
    }
    result
    // let set: std::collections::HashSet<_> = nums.into_iter().collect();
    //    if set.iter().any(|&n| n < k) {
    //        return -1;
    //    }
    //    set.into_iter().filter(|&n| n > k).count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, min_operations(vec![5, 2, 5, 4, 5], 2));
        assert_eq!(-1, min_operations(vec![2, 1, 2], 2));
        assert_eq!(4, min_operations(vec![9, 7, 5, 3], 1));
    }
}
