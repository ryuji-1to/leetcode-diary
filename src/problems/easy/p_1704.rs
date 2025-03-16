pub fn halves_are_alike(s: String) -> bool {
    let target = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let chars = s.chars().collect::<Vec<char>>();
    let a = &chars[0..chars.len() / 2];
    let b = &chars[chars.len() / 2..];
    let mut result = (0, 0);
    for i in 0..chars.len() / 2 {
        if target.contains(&a[i]) {
            result.0 += 1;
        }
        if target.contains(&b[i]) {
            result.1 += 1;
        }
    }
    // a.iter()
    //     .fold(0, |acc, x| if target.contains(x) { acc + 1 } else { acc })
    //     == b.iter()
    //         .fold(0, |acc, x| if target.contains(x) { acc + 1 } else { acc })
    result.0 == result.1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, halves_are_alike("book".to_string()));
        assert_eq!(false, halves_are_alike("textbook".to_string()));
    }
}
