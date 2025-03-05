pub fn di_string_match(s: String) -> Vec<i32> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len() as i32;
    let perm: Vec<i32> = (0..=n).collect();
    let mut stack = Vec::<i32>::new();
    let mut result = Vec::new();
    for (i, c) in chars.iter().enumerate() {
        match c {
            'I' => {
                result.push(perm[i]);
                result.extend(stack.iter().rev());
                stack.clear();
            }
            'D' => {
                stack.push(perm[i]);
            }
            _ => unreachable!(),
        }
    }
    result.push(*perm.last().unwrap());
    result.extend(stack.iter().rev());
    result
}

/*
// 常に最小値と最大値を追跡する
// I なら最小値を使用して増加
// D なら最大値を使用して減少
pub fn di_string_match(s: String) -> Vec<i32> {
    let n = s.len();
    let mut low = 0;
    let mut high = n as i32;
    let mut result = Vec::with_capacity(n + 1);

    for c in s.chars() {
        match c {
            'I' => {
                result.push(low);
                low += 1;
            }
            'D' => {
                result.push(high);
                high -= 1;
            }
            _ => unreachable!()
        }
    }

    // 最後の要素を追加
    result.push(low); // または high（どちらも同じ）
    result
}
*/

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         assert_eq!(vec![0, 4, 1, 3, 2], di_string_match("IDID".to_string()));
//         assert_eq!(vec![0, 1, 2, 3], di_string_match("III".to_string()));
//         assert_eq!(vec![3, 2, 0, 1], di_string_match("DDI".to_string()));
//     }
// }
