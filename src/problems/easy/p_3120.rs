pub fn number_of_special_chars(word: String) -> i32 {
    // let mut pair = vec![(0, 0); 26];
    // for c in word.chars() {
    //     if c.is_uppercase() {
    //         let index = c as u8 - b'A';
    //         pair[index as usize].1 = 1;
    //     } else {
    //         let index = c as u8 - b'a';
    //         pair[index as usize].0 = 1;
    //     }
    // }
    // pair.iter().filter(|&&x| x.0 == 1 && x.1 == 1).count() as i32

    let mut lowercase_mask = 0u32; // 小文字の存在を記録
    let mut uppercase_mask = 0u32; // 大文字の存在を記録

    for c in word.chars() {
        if c.is_ascii_alphabetic() {
            let index = (c.to_ascii_lowercase() as u8 - b'a') as u32;
            if index < 26 {
                if c.is_uppercase() {
                    uppercase_mask |= 1 << index;
                } else {
                    lowercase_mask |= 1 << index;
                }
            }
        }
    }

    // 両方のマスクでセットされているビットを数える
    (lowercase_mask & uppercase_mask).count_ones() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, number_of_special_chars("aaAbcBC".to_string()));
        assert_eq!(0, number_of_special_chars("abc".to_string()));
        assert_eq!(1, number_of_special_chars("abBCab".to_string()));
    }
}
