pub fn min_operations(boxes: String) -> Vec<i32> {
    // let mut idx_1 = Vec::new();
    // let mut result = Vec::new();
    // for (i, v) in boxes.chars().enumerate() {
    //     if v == '1' {
    //         idx_1.push(i);
    //     }
    // }
    // for i in 0..boxes.chars().count() {
    //     let count = idx_1.iter().fold(0, |acc, &x| acc + x.abs_diff(i));
    //     result.push(count as i32);
    // }
    // result
    let mut result = Vec::<i32>::new();
    let mut count_1 = 0;
    let mut sum_op_1 = 0;
    let chars = boxes.chars().collect::<Vec<_>>();
    let n = boxes.len();
    for i in (0..n) {
        sum_op_1 += count_1;
        result.push(sum_op_1);
        if chars[i] == '1' {
            count_1 += 1;
        }
    }
    let mut count_2 = 0;
    let mut sum_op_2 = 0;
    for i in (0..n).rev() {
        sum_op_2 += count_2;
        result[i] += sum_op_2;
        if chars[i] == '1' {
            count_2 += 1;
        }
    }
    result
}

// 左と右からの1の累積数の和
// "001011"
// [2,4,5]
// [11,8,5,4,3,4]
// -> [0,0,0,1,2,4]
// <- [11,8,5,3,1,0]

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![1, 1, 3], min_operations("110".to_string()));
        assert_eq!(
            vec![11, 8, 5, 4, 3, 4],
            min_operations("001011".to_string())
        );
    }
}
