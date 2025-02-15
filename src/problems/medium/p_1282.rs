use std::collections::HashMap;

pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    // let mut result = Vec::new();

    // for i in 0..group_sizes.len() {
    //     map.entry(group_sizes[i])
    //         .or_insert_with(Vec::new)
    //         .push(i as i32);
    // }

    let mut map = HashMap::<i32, Vec<i32>>::new();

    group_sizes.iter().enumerate().for_each(|(i, &size)| {
        map.entry(size).or_insert_with(Vec::new).push(i as i32);
    });

    map.iter()
        .flat_map(|(&key, value)| value.chunks(key as usize).map(|v| v.to_vec()))
        .collect()
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         assert_eq!(
//             vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]],
//             group_the_people(vec![3, 3, 3, 3, 3, 1, 3])
//         );
//         assert_eq!(
//             vec![vec![1], vec![0, 5], vec![2, 3, 4]],
//             group_the_people(vec![2, 1, 3, 3, 3, 2])
//         );
//     }
// }
