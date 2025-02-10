pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
    commands.iter().fold(0, |acc, x| match x.as_str() {
        "UP" => acc - n,
        "DOWN" => acc + n,
        "RIGHT" => acc + 1,
        "LEFT" => acc - 1,
        _ => acc,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            final_position_of_snake(2, vec!["RIGHT".to_string(), "DOWN".to_string()])
        );
        assert_eq!(
            1,
            final_position_of_snake(
                3,
                vec!["DOWN".to_string(), "RIGHT".to_string(), "UP".to_string()]
            )
        );
    }
}
