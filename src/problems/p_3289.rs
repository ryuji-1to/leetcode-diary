use std::collections::HashMap;

pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::<i32, ()>::new();
    let mut result = Vec::<i32>::new();

    for &v in &nums {
        if map.contains_key(&v) {
            result.push(v);
            continue;
        }
        map.insert(v, ());
    }
    result
}
