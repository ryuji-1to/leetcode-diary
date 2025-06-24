pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut magazine_map =
        magazine
            .chars()
            .fold(std::collections::HashMap::new(), |mut acc, ch| {
                *acc.entry(ch).or_insert(0) += 1;
                acc
            });
    // let ransom_note_map =
    //     ransom_note
    //         .chars()
    //         .fold(std::collections::HashMap::new(), |mut acc, ch| {
    //             *acc.entry(ch).or_insert(0) += 1;
    //             acc
    //         });
    //
    // for x in ransom_note_map {
    //     let target = *magazine_map.get(&x.0).unwrap_or(&0);
    //     if x.1 > target {
    //         return false;
    //     }
    // }
    // true

    for ch in ransom_note.chars() {
        match magazine_map.get_mut(&ch) {
            Some(count) if *count > 0 => *count -= 1,
            _ => return false,
        }
    }
    true
}

// impl Solution {
//     pub fn can_construct(ransom_note: String, magazine: String) -> bool {
//         let mut counter = vec![0i32; 26];
//         for ch in magazine.as_bytes() {
//             counter[(ch - b'a') as usize] += 1;
//         }
//
//         for ch in ransom_note.as_bytes() {
//             if counter[(ch - b'a') as usize] == 0 {
//                 return false;
//             }
//             counter[(ch - b'a') as usize] -= 1;
//         }
//
//         true
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(false, can_construct("a".to_string(), "b".to_string()));
        assert_eq!(false, can_construct("aa".to_string(), "ab".to_string()));
        assert_eq!(true, can_construct("aa".to_string(), "aab".to_string()));
    }
}
