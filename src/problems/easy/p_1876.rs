use std::collections::HashSet;

pub fn count_good_substrings(s: String) -> i32 {
    // s.chars()
    //     .collect::<Vec<char>>()
    //     .windows(3)
    //     .fold(0, |acc, xs| {
    //         if xs.len() == xs.iter().collect::<HashSet<_>>().len() {
    //             acc + 1
    //         } else {
    //             acc
    //         }
    //     })
    s.as_bytes()
        .windows(3)
        .filter(|&w| w.iter().collect::<HashSet<_>>().len() == 3)
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, count_good_substrings("xyzzaz".to_string()));
        assert_eq!(4, count_good_substrings("aababcabc".to_string()));
    }
}
