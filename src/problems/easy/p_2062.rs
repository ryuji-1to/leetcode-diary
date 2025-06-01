pub fn count_vowel_substrings(word: String) -> i32 {
    if word.len() < 5 {
        return 0;
    }

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result = 0;
    let words: Vec<char> = word.chars().collect();

    for i in 0..words.len() - 4 {
        if !vowels.contains(&words[i]) {
            continue;
        }
        for j in i + 4..words.len() {
            if !vowels.contains(&words[j]) {
                break;
            }
            if vowels.iter().all(|&c| word[i..=j].contains(c))
                && word[i..=j].chars().all(|c| vowels.contains(&c))
            {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, count_vowel_substrings("aeiouu".to_string()));
        assert_eq!(0, count_vowel_substrings("unicornarihan".to_string()));
        assert_eq!(7, count_vowel_substrings("cuaieuouac".to_string()));
        assert_eq!(0, count_vowel_substrings("bbaeixoubb".to_string()));
    }
}
