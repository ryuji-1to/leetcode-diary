use std::collections::HashMap;

pub fn decode_message(key: String, message: String) -> String {
    let m = (b'a'..=b'z').map(|c| c as char).collect::<Vec<_>>();
    let mut m_ = Vec::<char>::new();
    key.split_whitespace()
        .collect::<String>()
        .chars()
        .for_each(|x| {
            if !m_.contains(&x) {
                m_.push(x);
            }
        });
    let map = m_.iter().zip(m).collect::<HashMap<_, _>>();
    message
        .chars()
        .map(|from| match map.get(&from) {
            Some(v) => *v,
            None => ' ',
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            String::from("this is a secret"),
            decode_message(
                String::from("the quick brown fox jumps over the lazy dog"),
                String::from("vkbs bs t suepuv")
            )
        );
        assert_eq!(
            String::from("the five boxing wizards jump quickly"),
            decode_message(
                String::from("eljuxhpwwnyrdgtqkviszcfmabo"),
                String::from("zwx hnfx lqantp mnoeius ycgk vcnjrdb")
            )
        );
    }
}
