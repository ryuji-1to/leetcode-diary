pub fn final_string(s: String) -> String {
    s.chars()
        .fold(vec![], |mut acc, x| {
            if x == 'i' {
                acc.reverse();
            } else {
                acc.push(x);
            }
            acc
        })
        .iter()
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("rtsng".to_string(), final_string("string".to_string()));
        assert_eq!("ponter".to_string(), final_string("poiinter".to_string()));
    }
}
