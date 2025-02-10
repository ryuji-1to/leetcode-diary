use std::collections::HashMap;

pub fn find_permutation_difference(s: String, t: String) -> i32 {
    let mut map = HashMap::<char, usize>::new();
    for (i, v) in s.chars().enumerate() {
        map.insert(v, i);
    }
    let result = t
        .chars()
        .enumerate()
        .fold(0, |acc, (i, v)| match map.get(&v) {
            Some(&value) => acc + value.abs_diff(i),
            None => acc,
        });
    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            find_permutation_difference("abc".to_string(), "bac".to_string())
        );
        assert_eq!(
            12,
            find_permutation_difference("abcde".to_string(), "edbac".to_string())
        );
    }
}
