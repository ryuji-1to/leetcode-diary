pub fn min_time_to_type(word: String) -> i32 {
    let typewriter = ('a'..='z').collect::<Vec<_>>();
    let mut current = 0;
    word.chars()
        .map(|c| {
            let position = typewriter.iter().position(|&_c| _c == c).unwrap();
            let diff1 = position.abs_diff(current);
            let diff2 = typewriter.len() - diff1;
            let seconds = diff1.min(diff2) + 1;
            current = position;
            seconds as i32
        })
        .sum()
    /*
    word.as_bytes()
        .iter()
        .fold((word.len() as i32, b'a'), |(sum, prev), &b| {
            let steps = (prev as i32 - b as i32).abs();
            (sum + steps.min(26 - steps), b)
        })
        .0
    */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5, min_time_to_type("abc".to_string()));
        assert_eq!(7, min_time_to_type("bza".to_string()));
        assert_eq!(34, min_time_to_type("zjpc".to_string()));
    }
}
