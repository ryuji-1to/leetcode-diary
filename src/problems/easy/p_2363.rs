pub fn merge_similar_items(mut items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    items1.extend(items2);
    let mut tmp = vec![0; 1000];
    for i in 0..items1.len() {
        let key = items1[i][0] - 1;
        let value = items1[i][1];
        tmp[key as usize] += value;
    }
    tmp.iter()
        .enumerate()
        .map(|(i, &v)| vec![(i + 1) as i32, v])
        .filter(|x| x[1] > 0)
        .collect::<Vec<Vec<i32>>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![1, 6], vec![3, 9], vec![4, 5]],
            merge_similar_items(
                vec![vec![1, 1], vec![4, 5], vec![3, 8]],
                vec![vec![3, 1], vec![1, 5]]
            )
        );
        assert_eq!(
            vec![vec![1, 4], vec![2, 4], vec![3, 4]],
            merge_similar_items(
                vec![vec![1, 1], vec![3, 2], vec![2, 3]],
                vec![vec![2, 1], vec![3, 2], vec![1, 3]]
            )
        );
    }
}
