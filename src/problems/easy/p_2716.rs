use std::collections::HashSet;

pub fn minimized_string_length(s: String) -> i32 {
    s.chars().collect::<HashSet<char>>().len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, minimized_string_length("aaabc".to_string()));
        assert_eq!(3, minimized_string_length("cbbd".to_string()));
        assert_eq!(4, minimized_string_length("baadccab".to_string()));
    }
}
