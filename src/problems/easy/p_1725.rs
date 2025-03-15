use std::collections::HashMap;

pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    let min_length_list = rectangles
        .iter()
        .map(|x| *x.iter().min().unwrap())
        .collect::<Vec<_>>();
    let max_len = min_length_list.iter().max().unwrap();
    min_length_list.iter().filter(|&x| x == max_len).count() as i32
    // let mut map = HashMap::new();
    // for v in min_length_list {
    //     *map.entry(v).or_insert(0) += 1;
    // }
    // map.into_iter().max_by_key(|&(_, v)| v).unwrap().1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]])
        );
        assert_eq!(
            3,
            count_good_rectangles(vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]])
        );
    }
}
