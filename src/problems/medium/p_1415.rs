pub fn get_happy_string(n: i32, k: i32) -> String {
    // if n == 1 {
    //     return ["a".to_string(), "b".to_string(), "c".to_string()]
    //         .get((k - 1) as usize)
    //         .cloned()
    //         .unwrap_or("".to_string());
    // };
    //
    // let mut result: Vec<String> = Vec::new();
    // fn dfs(index: i32, end: i32, mut current: String, result: &mut Vec<String>, next_char: char) {
    //     current.push(next_char);
    //     if index == end {
    //         result.push(current);
    //         return;
    //     }
    //     if next_char == 'a' {
    //         dfs(index + 1, end, current.clone(), result, 'b');
    //         dfs(index + 1, end, current, result, 'c');
    //     } else if next_char == 'b' {
    //         dfs(index + 1, end, current.clone(), result, 'a');
    //         dfs(index + 1, end, current, result, 'c');
    //     } else {
    //         dfs(index + 1, end, current.clone(), result, 'a');
    //         dfs(index + 1, end, current, result, 'b');
    //     }
    // }
    //
    // dfs(1, n - 1, "a".to_string(), &mut result, 'b');
    // dfs(1, n - 1, "a".to_string(), &mut result, 'c');
    // dfs(1, n - 1, "b".to_string(), &mut result, 'a');
    // dfs(1, n - 1, "b".to_string(), &mut result, 'c');
    // dfs(1, n - 1, "c".to_string(), &mut result, 'a');
    // dfs(1, n - 1, "c".to_string(), &mut result, 'b');
    //
    // match result.get((k - 1) as usize) {
    //     Some(v) => v.to_string(),
    //     None => "".to_string(),
    // }

    let mut result: Vec<String> = Vec::new();

    fn dfs(index: i32, end: i32, current: &mut String, result: &mut Vec<String>) {
        if index == end {
            result.push(current.clone());
            return;
        }

        for &ch in &['a', 'b', 'c'] {
            if current.chars().last() == Some(ch) {
                continue;
            }
            current.push(ch);
            dfs(index + 1, end, current, result);
            current.pop(); // 元の状態に戻す
        }
    }

    for &ch in &['a', 'b', 'c'] {
        let mut current = String::new();
        current.push(ch);
        dfs(1, n, &mut current, &mut result);
    }

    result
        .get((k - 1) as usize)
        .cloned()
        .unwrap_or("".to_string())
}

/**
* other solution??
pub fn get_happy_string(n: i32, k: i32) -> String {
    let count = 3 * 2_i32.pow((n - 1) as u32);
    if k > count {
        return "".to_string();
    }

    let mut result = String::new();
    let mut chars = vec!['a', 'b', 'c'];
    let mut k = k - 1; // 0-indexed にする

    for i in 0..n {
        let group_size = 2_i32.pow((n - i - 1) as u32); // その桁で何個のグループができるか
        let index = (k / group_size) as usize; // k がどの範囲に属するか
        let next_char = chars[index]; // 選ばれる文字
        result.push(next_char);
        k %= group_size; // 次の桁での位置を更新
        chars.retain(|&c| c != next_char); // 直前の文字と同じものを除外
    }

    result
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("c".to_string(), get_happy_string(1, 3));
        assert_eq!("".to_string(), get_happy_string(1, 4));
        assert_eq!("cab".to_string(), get_happy_string(3, 9));
    }
}
