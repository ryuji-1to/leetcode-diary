pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for v in &nums1 {
        if (nums2.contains(v) || nums3.contains(v)) && !result.contains(v) {
            result.push(*v);
        }
    }
    for v in &nums2 {
        if nums3.contains(v) && !result.contains(v) {
            result.push(*v);
        }
    }
    result
    /*
    use std::collections::{HashSet, HashMap};
    // Convert to sets to remove duplicates
    let set1: HashSet<i32> = nums1.into_iter().collect();
    let set2: HashSet<i32> = nums2.into_iter().collect();
    let set3: HashSet<i32> = nums3.into_iter().collect();

    // Count occurrences in different sets
    let mut count: HashMap<i32, u8> = HashMap::new();

    for num in set1 {
        *count.entry(num).or_insert(0) |= 1;
    }

    for num in set2 {
        *count.entry(num).or_insert(0) |= 2;
    }

    for num in set3 {
        *count.entry(num).or_insert(0) |= 4;
    }

    // Filter numbers that appear in at least two sets
    count.into_iter()
        .filter(|&(_, v)| v > 1)  // v == 3 (1|2), 5 (1|4), or 6 (2|4)
        .map(|(k, _)| k)
        .collect()
    */
}
