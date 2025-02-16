pub fn valid_strings(n: i32) -> Vec<String> {
    let mut result = Vec::<String>::new();
    // fn dfs(index: i32, end: i32, result: &mut Vec<String>, current: String, next_c: char) {
    //     let mut current = current.clone();
    //     current.push(next_c);
    //     if index == end {
    //         result.push(current.clone());
    //         return;
    //     }
    //     dfs(index + 1, end, result, current.clone(), '1');
    //     if next_c == '1' {
    //         dfs(index + 1, end, result, current.clone(), '0');
    //     }
    // }
    // dfs(1, n, &mut result, "".to_string(), '0');
    // dfs(1, n, &mut result, "".to_string(), '1');

    // better
    fn dfs(index: i32, end: i32, result: &mut Vec<String>, current: &mut Vec<char>, next_c: char) {
        current.push(next_c);
        if index == end {
            result.push(current.iter().collect());
        } else {
            dfs(index + 1, end, result, current, '1');
            if next_c == '1' {
                dfs(index + 1, end, result, current, '0');
            }
        }
        // for back tracking
        current.pop();
    }
    let mut current = Vec::new();
    dfs(1, n, &mut result, &mut current, '0');
    dfs(1, n, &mut result, &mut current, '1');
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let expected = vec![
            "010".to_string(),
            "011".to_string(),
            "110".to_string(),
            "101".to_string(),
            "111".to_string(),
        ];
        let actual = valid_strings(3);
        for e in expected {
            assert!(actual.contains(&e));
        }

        let expected = vec!["0".to_string(), "1".to_string()];
        let actual = valid_strings(1);
        for e in expected {
            assert!(actual.contains(&e));
        }
    }
}
