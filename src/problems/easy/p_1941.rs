pub fn are_occurrences_equal(s: String) -> bool {
    let mut tmp = vec![0; 26];
    for c in s.chars() {
        tmp[((c as u8) - b'a') as usize] += 1;
    }
    let filtered = tmp.into_iter().filter(|&c| c != 0).collect::<Vec<i32>>();
    filtered.iter().all(|&x| x == filtered[0])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, are_occurrences_equal("abacbc".to_string()));
        assert_eq!(false, are_occurrences_equal("aaabb".to_string()));
    }
}
