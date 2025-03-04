pub fn minimum_chairs(s: String) -> i32 {
    let mut result = 0;
    let mut now = 0;
    for c in s.chars() {
        if c == 'E' {
            now += 1;
            result = result.max(now);
        }
        if c == 'L' {
            now -= 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(7, minimum_chairs("EEEEEEE".to_string()));
        assert_eq!(2, minimum_chairs("ELELEEL".to_string()));
        assert_eq!(3, minimum_chairs("ELEELEELLL".to_string()));
    }
}
