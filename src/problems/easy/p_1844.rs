pub fn replace_digits(s: String) -> String {
    // let map = ('a'..='z').collect::<Vec<char>>();
    //
    // s.chars()
    //     .collect::<Vec<_>>()
    //     .chunks(2)
    //     .map(|x| {
    //         let index = x.get(1);
    //         match index {
    //             Some(v) => {
    //                 let c_index = map.iter().position(|&c| c == x[0]).unwrap();
    //                 let shifted = map[c_index + v.to_digit(10).unwrap() as usize];
    //                 [x[0], shifted].iter().collect::<String>()
    //             }
    //             None => x[0].to_string(),
    //         }
    //     })
    //     .collect::<String>()
    s.as_bytes()
        .chunks(2)
        .flat_map(|chunk| {
            let c = chunk[0] as char;
            let shifted = chunk.get(1).map(|&d| ((c as u8) + (d - b'0')) as char);
            [Some(c), shifted].into_iter().flatten()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("abcdef".to_string(), replace_digits("a1c1e1".to_string()));
        assert_eq!(
            "abbdcfdhe".to_string(),
            replace_digits("a1b2c3d4e".to_string())
        );
    }
}
