pub fn max_frq_sum(s: String) -> i32 {
    let mut consonants = vec![0; 26];
    let mut max = (0, 0);

    for c in s.chars() {
        let n = (c as u8 - b'a') as usize;
        consonants[n] += 1;
        let target = consonants[n];
        if "aeiou".contains(c) {
            max.0 = max.0.max(target);
        } else {
            max.1 = max.1.max(target);
        }
    }
    max.0 + max.1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(6, max_frq_sum("successes".to_string()));
        assert_eq!(3, max_frq_sum("aeiaeia".to_string()));
    }
}
