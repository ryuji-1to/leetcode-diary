pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
    let mut tmp = vec![0; 100];
    for v in nums {
        for i in v[0]..=v[1] {
            tmp[(i - 1) as usize] = 1;
        }
    }
    tmp.iter().filter(|&&x| x == 1).count() as i32
    // // Rangeの区間を集合として使用
    // let mut points = std::collections::HashSet::new();
    //
    // // 各区間の点を集合に追加
    // for range in nums {
    //     let start = range[0];
    //     let end = range[1];
    //
    //     // 区間内の各点を集合に追加
    //     for point in start..=end {
    //         points.insert(point);
    //     }
    // }
    //
    // // 集合のサイズを返す（ユニークな点の数）
    // points.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            7,
            number_of_points(vec![vec![3, 6], vec![1, 5], vec![4, 7]])
        );
        assert_eq!(7, number_of_points(vec![vec![1, 3], vec![5, 8]]));
        assert_eq!(
            8,
            number_of_points(vec![vec![4, 4], vec![9, 10], vec![9, 10], vec![3, 8]])
        );
    }
}
