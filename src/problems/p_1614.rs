pub fn max_depth(s: String) -> i32 {
    let mut count = 0;
    let mut result = 0;
    for v in s.chars() {
        if v == '(' {
            count += 1;
            result = if count > result { count } else { result }
        } else if v == ')' {
            count -= 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, max_depth("(1+(2*3)+((8/4))+1".to_string()));
        assert_eq!(3, max_depth("(1)+((2))+(((3)))".to_string()));
        assert_eq!(3, max_depth("()(())((()()))".to_string()));
    }
}
