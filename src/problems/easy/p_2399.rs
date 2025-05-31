pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
    let mut tmp = vec![0; 26];
    for (i, c) in s.chars().enumerate() {
        let index = (c as u8 - b'a') as usize;
        if tmp[index] == 0 {
            tmp[index] = i + 1;
            continue;
        }
        tmp[index] = i - tmp[index];
        if tmp[index] != distance[index] as usize {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            check_distances(
                "abaccb".to_string(),
                vec![1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            )
        );
        assert_eq!(
            false,
            check_distances(
                "aa".to_string(),
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            )
        );
    }
}
