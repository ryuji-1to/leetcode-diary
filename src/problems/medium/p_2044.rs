// pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
//     let max_length = (2 as u32).pow(nums.len() as u32) - 1;
//     let maximum = nums.iter().fold(0, |acc, x| acc | x);
//     if nums.len() == 1 {
//         return 1;
//     }
//     find_target(nums[1..nums.len()].to_vec(), maximum, nums[0], 0)
// }
// // recursible
// fn find_target(nums: Vec<i32>, maximum: i32, current: i32, total: i32) -> i32 {
//     let mut total = total;
//     if nums.is_empty() {
//         if current == maximum {
//             total += 1;
//         }
//         return total;
//     }
//     for &v in &nums {
//         if current | v == maximum {
//             total += 1;
//         }
//     }
//     let next = nums[0];
//     let next_nums = nums[1..nums.len()].to_vec();
//     return find_target(next_nums.clone(), maximum, current | next, total)
//         + find_target(next_nums, maximum, next, 0);
// }

pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let max_or = nums.iter().fold(0, |acc, x| acc | x);
    let mut count = 0;

    fn dfs(index: usize, current_or: i32, nums: &Vec<i32>, max_or: i32, count: &mut i32) {
        if index == nums.len() {
            if current_or == max_or {
                *count += 1;
            }
            return;
        }
        // ① 現在の要素を選ぶ
        dfs(index + 1, current_or | nums[index], nums, max_or, count);
        // ② 現在の要素を選ばない
        dfs(index + 1, current_or, nums, max_or, count);
    }

    dfs(0, 0, &nums, max_or, &mut count);
    count
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, count_max_or_subsets(vec![3, 1]));
        assert_eq!(7, count_max_or_subsets(vec![2, 2, 2]));
        assert_eq!(6, count_max_or_subsets(vec![3, 2, 1, 5]));
    }
}
