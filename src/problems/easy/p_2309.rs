pub fn greatest_letter(s: String) -> String {
    let mut tmp = vec![(' ', ' '); 26];
    for c in s.chars() {
        if !c.is_ascii_alphabetic() {
            continue;
        }
        let lower = c.to_lowercase().next().unwrap();
        let idx = (lower as u8 - b'a') as usize;
        if lower == c {
            tmp[idx].0 = c;
        } else {
            tmp[idx].1 = c;
        }
    }
    let result = tmp.into_iter().filter(|&x| x.0 != ' ' && x.1 != ' ').last();
    match result {
        Some((_, c)) => c.to_string(),
        None => "".to_string(),
    }
}

// pub fn greatest_letter_2(s: String) -> String {
//     match ('A'..='Z')
//         .into_iter()
//         .rev()
//         .find(|&c| s.contains(c) && s.contains(c.to_ascii_lowercase()))
//     {
//         Some(c) => c.to_string(),
//         _ => "".to_string(),
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("E".to_string(), greatest_letter("lEeTc0dE".to_string()));
        assert_eq!("R".to_string(), greatest_letter("arRAzFif".to_string()));
        assert_eq!("".to_string(), greatest_letter("AbCdEfGhIjK".to_string()));
    }
}
