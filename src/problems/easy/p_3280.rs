pub fn convert_date_to_binary(date: String) -> String {
    // let splited = date.split("-").collect::<Vec<&str>>();
    // let mut d = Vec::<String>::new();
    // for &v in &splited {
    //     match v.parse::<usize>() {
    //         Ok(x) => d.push(format!("{:b}", x)),
    //         Err(x) => (),
    //     }
    // }
    // d.join("-")
    date.split("-")
        .filter_map(|v| v.parse::<usize>().ok().map(|x| format!("{:b}", x)))
        .collect::<Vec<String>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            convert_date_to_binary("2080-02-29".to_string()),
            "100000100000-10-11101".to_string()
        );
        assert_eq!(
            convert_date_to_binary("1900-01-01".to_string()),
            "11101101100-1-1".to_string()
        );
    }
}
