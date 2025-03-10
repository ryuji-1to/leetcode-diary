pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    // let mut flat: Vec<i32> = grid.into_iter().flatten().collect();
    // flat.sort_unstable();
    // let mut result = vec![0; 2];
    // let mut tmp = 0;
    // for i in (1..=flat.len()) {
    //     if !flat.contains(&(i as i32)) {
    //         result[1] = i as i32;
    //     }
    //     if tmp == flat[i - 1] {
    //         result[0] = flat[i - 1];
    //     }
    //     tmp = flat[i - 1];
    // }
    // result
    let n = grid.len();
    let n_squared = n * n;
    // すべての数値の出現回数をカウントする
    let mut count = vec![0; n_squared + 1];
    // グリッドをフラット化してカウント
    for row in &grid {
        for &num in row {
            count[num as usize] += 1;
        }
    }
    
    let mut repeated = 0;
    let mut missing = 0;
    // 出現回数が2のものが繰り返し値、0のものが欠落値
    for i in 1..=n_squared {
        if count[i] == 2 {
            repeated = i as i32;
        } else if count[i] == 0 {
            missing = i as i32;
        }
    }
    
    vec![repeated, missing]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![2, 4],
            find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]])
        );
        assert_eq!(
            vec![9, 5],
            find_missing_and_repeated_values(vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]])
        );
    }
}
