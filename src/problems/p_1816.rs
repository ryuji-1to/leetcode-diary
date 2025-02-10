pub fn truncate_sentence(s: String, k: i32) -> String {
    s.split(" ").collect::<Vec<_>>()[0..(k as usize)].join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            String::from("Hello how are you"),
            truncate_sentence(String::from("Hello how are you Contestant"), 4)
        );
        assert_eq!(
            String::from("What is the solution"),
            truncate_sentence(String::from("What is the solution to this problem"), 4)
        );
        assert_eq!(
            String::from("chopper is not a tanuki"),
            truncate_sentence(String::from("chopper is not a tanuki"), 5)
        );
    }
}
