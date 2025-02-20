use std::collections::{HashMap, HashSet};

pub fn sum_counts(nums: Vec<i32>) -> i32 {
    let mut result = 0;

    // for i in 0..nums.len() {
    //     let mut freq = HashMap::new();
    //     let mut distinct_count = 0;
    //     for j in i..nums.len() {
    //         let num = nums[j];
    //         let count = freq.entry(num).or_insert(0);
    //         if *count == 0 {
    //             distinct_count += 1;
    //         }
    //         *count += 1;
    //         result += distinct_count * distinct_count;
    //     }
    // }

    for i in 0..nums.len() {
        let mut freq = HashSet::new();
        for j in i..nums.len() {
            let num = nums[j];
            freq.insert(num);
            result += freq.len().pow(2) as i32;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(15, sum_counts(vec![1, 2, 1]));
        assert_eq!(3, sum_counts(vec![1, 1]));
    }
}
