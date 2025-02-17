use std::{cmp::max, collections::HashMap};

// pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     let mut map = HashMap::new();
//     for v in nums {
//         *map.entry(v).or_insert(0) += 1;
//     }
//     let max_count = map.values().max().unwrap();
//     (0..*max_count)
//         .map(|x| {
//             let mut _v = Vec::new();
//             for (&k, &v) in &map.clone() {
//                 if v > 0 {
//                     _v.push(k);
//                     map.insert(k, v - 1);
//                 }
//             }
//             _v
//         })
//         .collect::<Vec<Vec<i32>>>()
// }

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map = HashMap::new();
    for v in nums {
        *map.entry(v).or_insert(0) += 1;
    }
    let max_count = map.values().max().unwrap();

    let mut result = vec![vec![]; *max_count];

    for (num, count) in map {
        for i in 0..count {
            result[i].push(num);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut expected = vec![vec![1, 3, 4, 2], vec![1, 3], vec![1]];
        let mut actual = find_matrix(vec![1, 3, 4, 1, 2, 3, 1]);
        for (e, v) in expected.iter_mut().zip(actual.iter_mut()) {
            e.sort_unstable();
            v.sort_unstable();
            assert_eq!(e, v);
        }

        let mut expected = vec![vec![1, 3, 4, 2]];
        let mut actual = find_matrix(vec![1, 2, 3, 4]);
        for (e, v) in expected.iter_mut().zip(actual.iter_mut()) {
            e.sort_unstable();
            v.sort_unstable();
            assert_eq!(e, v);
        }
    }
}
