pub fn min_length(s: String) -> i32 {
    // let mut s = s;
    // while s.contains(&"AB") || s.contains(&"CD") {
    //     s = s.replace("AB", "");
    //     s = s.replace("CD", "");
    // }
    // s.len() as i32
    let mut stack = Vec::with_capacity(s.len());
    for c in s.chars() {
        if let Some(&last) = stack.last() {
            if (last == 'A' && c == 'B') || (last == 'C' && c == 'D') {
                stack.pop();
                continue;
            }
        }
        stack.push(c);
    }
    stack.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, min_length("ABFCACDB".to_string()));
        assert_eq!(5, min_length("ACBBD".to_string()));
    }
}
