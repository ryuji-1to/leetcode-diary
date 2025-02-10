pub fn sort_sentence(s: String) -> String {
    let mut splited = s.split(" ").collect::<Vec<_>>();
    splited.sort_unstable_by_key(|&s| s.chars().last().unwrap().to_digit(10));
    splited
        .iter()
        .map(|&c| &c[..c.len() - 1])
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "This is a sentence".to_string(),
            sort_sentence("is2 sentence4 This1 a3".to_string())
        );
        assert_eq!(
            "Me Myself and I".to_string(),
            sort_sentence("Myself2 Me1 I4 and3".to_string())
        );
    }
}
