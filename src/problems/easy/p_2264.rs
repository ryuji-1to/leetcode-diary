use std::fmt::format;

pub fn largest_good_integer(num: String) -> String {
    num.chars()
        .collect::<Vec<_>>()
        .windows(3)
        .filter_map(|x| {
            if x[0] == x[1] && x[1] == x[2] {
                Some(x[0])
            } else {
                None
            }
        })
        .max()
        .map(|c| format!("{}{}{}", c, c, c))
        .unwrap_or("".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "777".to_string(),
            largest_good_integer("6777133339".to_string())
        );
        assert_eq!(
            "000".to_string(),
            largest_good_integer("2300019".to_string())
        );
        assert_eq!("".to_string(), largest_good_integer("42352338".to_string()));
    }
}
