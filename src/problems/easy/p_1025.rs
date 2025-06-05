pub fn divisor_game(mut n: i32) -> bool {
    let mut user = "Alice";
    while n > 1 {
        for x in (1..n) {
            if n % x == 0 {
                n -= x;
                user = if user == "Alice" { "Bob" } else { "Alice" };
                break;
            }
        }
    }
    user != "Alice"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, divisor_game(2));
        assert_eq!(false, divisor_game(3));
        assert_eq!(true, divisor_game(4));
    }
}
