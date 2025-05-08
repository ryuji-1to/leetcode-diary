pub fn similar_pairs(words: Vec<String>) -> i32 {
    // let words = words
    //     .into_iter()
    //     .map(|s| {
    //         let mut s = s
    //             .chars()
    //             .collect::<std::collections::HashSet<_>>()
    //             .into_iter()
    //             .collect::<Vec<_>>();
    //         s.sort_unstable();
    //         s.iter().collect::<String>()
    //     })
    //     .collect::<Vec<String>>();
    // let mut result = 0;
    // for i in 0..words.len() - 1 {
    //     result += (&words[i + 1..])
    //         .into_iter()
    //         .filter(|&x| x == &words[i])
    //         .count() as i32;
    // }
    // result
    let words = words
        .into_iter()
        .map(|s| {
            let mut tmp = vec!['0'; 26];
            for c in s.chars() {
                let index = c as u8 - b'a';
                if tmp[index as usize] == '0' {
                    tmp[index as usize] = (index + 1) as char;
                };
            }
            tmp.iter().collect::<String>()
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    for i in 0..words.len() - 1 {
        result += words[i + 1..]
            .into_iter()
            .filter(|&x| x == &words[i])
            .count() as i32;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            similar_pairs(vec![
                "aba".to_string(),
                "aabb".to_string(),
                "abcd".to_string(),
                "bac".to_string(),
                "aabc".to_string()
            ])
        );
        assert_eq!(
            3,
            similar_pairs(vec!["aabb".to_string(), "ab".to_string(), "ba".to_string()])
        );
        assert_eq!(
            0,
            similar_pairs(vec![
                "nba".to_string(),
                "cba".to_string(),
                "dba".to_string()
            ])
        );
        assert_eq!(
            10,
            similar_pairs(vec![
                "acaeeccaaedabeddddae".to_string(),
                "bedbbeaabbbaaeabbdea".to_string(),
                "eeaadeddbebcdeaecccd".to_string(),
                "edccaaeeaaceaaeedbce".to_string(),
                "eadacedabddebdeebddb".to_string(),
                "ecdeecbcdeceabacadac".to_string(),
                "becbddbdbeededeecebb".to_string()
            ])
        )
    }
}
