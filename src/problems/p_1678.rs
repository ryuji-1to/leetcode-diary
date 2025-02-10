pub fn interpret(command: String) -> String {
    command.replace("()", "o").replace("(al)", "al")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(String::from("Goal"), interpret(String::from("G()(al)")));
        assert_eq!(
            String::from("Gooooal"),
            interpret(String::from("G()()()()(al)"))
        );
        assert_eq!(
            String::from("alGalooG"),
            interpret(String::from("(al)G(al)()()G"))
        );
    }
}
