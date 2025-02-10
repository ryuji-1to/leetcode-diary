pub fn check_if_pangram(sentence: String) -> bool {
    // let mut result = vec![];
    // for v in sentence.chars() {
    //     if !result.contains(&v) {
    //         result.push(v);
    //     }
    // }
    // result.len() == ('a'..='z').collect::<Vec<char>>().len()

    // sentence
    //     .chars()
    //     .fold((vec![], false), |(mut acc, result), c| {
    //         if !acc.contains(&c) {
    //             acc.push(c);
    //         }
    //         if acc.len() == 26 {
    //             return (acc, true);
    //         }
    //         (acc, false)
    //     })
    //     .1
    let mut result = Vec::new();
    sentence.chars().any(|c| {
        if !result.contains(&c) {
            result.push(c);
        }
        result.len() == 26
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string())
        );
        assert_eq!(false, check_if_pangram("leetcode".to_string()));
    }
}
