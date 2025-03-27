pub fn sort_string(s: String) -> String {
    let mut tmp = vec![0; 26];
    for c in s.chars() {
        tmp[(c as u8 - b'a') as usize] += 1;
    }
    let maximum = *tmp.iter().max().unwrap();
    let mut result = String::new();

    for i in 0..maximum {
        let mut s = String::new();
        for (j, v) in tmp.iter_mut().enumerate() {
            if *v > 0 {
                if i % 2 == 0 {
                    s.push((b'a' + j as u8) as char);
                } else {
                    s.insert(0, (b'a' + j as u8) as char);
                }
                *v -= 1;
            }
        }
        result.push_str(s.as_str());
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "abccbaabccba".to_string(),
            sort_string("aaaabbbbcccc".to_string())
        );
        assert_eq!("art".to_string(), sort_string("rat".to_string()));
    }
}
