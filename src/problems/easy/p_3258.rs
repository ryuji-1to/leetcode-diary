pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut result = 0;
    for i in 0..n {
        let mut ones = 0;
        let mut zeros = 0;
        for j in i..n {
            let target = chars[j];
            match target {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => panic!("unexpected"),
            }
            if ones <= k || zeros <= k {
                result += 1;
            } else {
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(12, count_k_constraint_substrings("10101".to_string(), 1));
        assert_eq!(25, count_k_constraint_substrings("1010101".to_string(), 2));
        assert_eq!(15, count_k_constraint_substrings("11111".to_string(), 1));
    }
}
