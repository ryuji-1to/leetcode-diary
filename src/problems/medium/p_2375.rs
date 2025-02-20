pub fn smallest_number(pattern: String) -> String {
    let mut freq = Vec::<u8>::with_capacity(pattern.len() + 1);
    let mut stack = Vec::<u8>::new();
    for (i, c) in pattern.chars().enumerate() {
        let num = (i + 1) as u8;
        match c {
            'I' => {
                freq.push(num);
                freq.extend(stack.iter().rev());
                stack.clear();
            }
            'D' => {
                stack.push(num);
            }
            _ => panic!("unexpected"),
        }
    }
    let last_num = pattern.len() + 1;
    stack.push(last_num as u8);
    freq.extend(stack.iter().rev());
    freq.iter().map(|x| x.to_string()).collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "123549876".to_string(),
            smallest_number("IIIDIDDD".to_string())
        );
        assert_eq!("4321".to_string(), smallest_number("DDD".to_string()));
    }
}
