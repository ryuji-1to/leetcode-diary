pub fn count_key_changes(s: String) -> i32 {
    let n: Vec<char> = s.to_lowercase().chars().collect();
    let mut result = 0;
    for i in 0..n.len() - 1 {
        if n[i] != n[i + 1] {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, count_key_changes("aAbBcC".to_string()));
        assert_eq!(0, count_key_changes("AaAaAaaA".to_string()));
    }
}
