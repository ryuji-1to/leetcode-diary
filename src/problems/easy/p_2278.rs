pub fn percentage_letter(s: String, letter: char) -> i32 {
    let count = s.chars().filter(|&c| c == letter).count();
    let percent = (count as f64 / s.len() as f64);
    (percent * 100.0) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(33, percentage_letter("foobar".into(), 'o'));
        assert_eq!(0, percentage_letter("jjjj".into(), 'k'));
    }
}
