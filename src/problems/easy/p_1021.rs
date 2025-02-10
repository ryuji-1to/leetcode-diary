pub fn remove_outer_parentheses(s: String) -> String {
    // let mut left = 0;
    // let mut right = 0;
    let mut count = 0;
    let mut pare = String::from("");
    for v in s.chars() {
        if v == '(' {
            // left += 1;
            // if left >= 2 {
            //     pare.push('(');
            // }
            if count > 0 {
                pare.push('(');
            }
            count += 1
        }
        if v == ')' {
            count -= 1;
            if count > 0 {
                pare.push(')');
            }
            // right += 1;
            // if right != left {
            //     pare.push(')');
            // }
        }
        // if right == left {
        //     left = 0;
        //     right = 0;
        // }
    }

    pare
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "()()()".to_string(),
            remove_outer_parentheses("(()())(())".to_string())
        );
        assert_eq!(
            "()()()()(())".to_string(),
            remove_outer_parentheses("(()())(())(()(()))".to_string())
        );
        assert_eq!(
            "()()()".to_string(),
            remove_outer_parentheses("(()())(())".to_string())
        );
    }
}
