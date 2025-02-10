use core::num;

pub fn artithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    nums.iter().fold(0, |acc, &x| {
        if nums.contains(&(x + diff)) && nums.contains(&(x + diff * 2)) {
            acc + 1
        } else {
            acc
        }
    })
    // let mut result = 0;
    // for i in 0..nums.len() {
    //     let target1 = nums[i] + diff;
    //     let target2 = target1 + diff;
    //     if !nums.contains(&target1) || !nums.contains(&target2) {
    //         continue;
    //     }
    //
    //
    //     // if !nums[i + 1..nums.len()].contains(&target) {
    //     //     continue;
    //     // }
    //     // let next_start = i + 1;
    //     // let id = nums[next_start..nums.len()]
    //     //     .iter()
    //     //     .position(|&x| x == target)
    //     //     .unwrap();
    //     // let target = nums[id + next_start] + diff;
    //     // if !nums[id + next_start..nums.len()].contains(&target) {
    //     //     continue;
    //     // }
    //     result += 1;
    // }
    // result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, artithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3));
        assert_eq!(2, artithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2));
    }
}
