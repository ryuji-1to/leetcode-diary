pub fn count_asterisks(s: String) -> i32 {
    s.split('|').enumerate().fold(0, |acc, (i, x)| {
        if i % 2 == 1 {
            acc
        } else {
            acc + x.matches('*').count() as i32
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, count_asterisks("l|*e*et|c**o|*de|".to_string()));
        assert_eq!(0, count_asterisks("iamprogrammer".to_string()));
        assert_eq!(5, count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string()));
    }
}
