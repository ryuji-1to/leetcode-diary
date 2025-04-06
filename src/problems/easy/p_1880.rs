pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
    let v = [first_word, second_word, target_word].map(|x| {
        x.bytes()
            .map(|x| (x - b'a').to_string())
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    });
    v[0] + v[1] == v[2]
    // let first_word_to_num = first_word
    //     .bytes()
    //     .map(|x| (x - b'a').to_string())
    //     .collect::<String>();
    // let second_word_to_num = second_word
    //     .bytes()
    //     .map(|x| (x - b'a').to_string())
    //     .collect::<String>();
    // let target_word_to_num = target_word
    //     .bytes()
    //     .map(|x| (x - b'a').to_string())
    //     .collect::<String>();

    // first_word_to_num.parse::<i32>().unwrap() + second_word_to_num.parse::<i32>().unwrap()
    //     == target_word_to_num.parse::<i32>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, is_sum_equal("acb".into(), "cba".into(), "cdb".into()));
        assert_eq!(false, is_sum_equal("aaa".into(), "a".into(), "aab".into()));
    }
}
