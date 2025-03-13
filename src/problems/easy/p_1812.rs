pub fn square_is_white(coordinates: String) -> bool {
    coordinates.chars().map(|x| x as u32).sum::<u32>() % 2 == 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(false, square_is_white("a1".to_string()));
        assert_eq!(true, square_is_white("h3".to_string()));
        assert_eq!(false, square_is_white("c7".to_string()));
    }
}
