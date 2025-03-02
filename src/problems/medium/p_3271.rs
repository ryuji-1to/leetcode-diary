pub fn string_hash(s: String, k: i32) -> String {
    let chars: Vec<char> = s.chars().collect();
    chars.chunks(k as usize).fold(String::new(), |mut acc, xs| {
        let hash_sum = xs
            .iter()
            .fold(0, |sum, &char| sum + (char as u8 - b'a') as i32);
        let hashed_char = (hash_sum % 26) as u8;
        let target = (b'a' + hashed_char) as char;
        acc.push(target);
        acc
    })

    // chars
    //     .chunks(k as usize)
    //     .map(|chunk| {
    //         let hash_sum: i32 = chunk.iter().map(|&c| (c as u8 - b'a') as i32).sum();
    //         ((hash_sum % 26) as u8 + b'a') as char
    //     })
    //     .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("bf".to_string(), string_hash("abcd".to_string(), 2));
        assert_eq!("i".to_string(), string_hash("mxz".to_string(), 3));
    }
}
