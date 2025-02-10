pub fn reverse_prefix(mut word: String, ch: char) -> String {
    // if !word.contains(ch) {
    //     return word;
    // }
    // let mut isOk = false;
    // let mut result = Vec::new();
    // for c in word.chars() {
    //     if c != ch && !isOk {
    //         result.insert(0, c);
    //     } else if c == ch && !isOk {
    //         result.insert(0, c);
    //         isOk = true
    //     } else {
    //         result.push(c);
    //     }
    // }
    // result.iter().collect::<String>()

    // if let Some(index) = word.find(ch) {
    //     let (prefix, rest) = word.split_at(index + 1);
    //     let reversed: String = prefix.chars().rev().collect();
    //     format!("{}{}", reversed, rest)
    // } else {
    //     word
    // }

    match word.find(ch) {
        Some(i) => {
            let (prefix, rest) = word.split_at(i + 1);
            let reversed: String = prefix.chars().rev().collect();
            format!("{}{}", reversed, rest)
        }
        None => word,
    }
    // if let Some(i) = word.find(ch) {
    //     unsafe { word.get_unchecked_mut(0..=i).as_bytes_mut().reverse() }
    // }
    // word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            String::from("dcbaefd"),
            reverse_prefix(String::from("abcdefd"), 'd')
        );
        assert_eq!(
            String::from("zxyxxe"),
            reverse_prefix(String::from("xyxzxe"), 'z')
        );
        assert_eq!(
            String::from("abcd"),
            reverse_prefix(String::from("abcd"), 'z')
        );
    }
}
