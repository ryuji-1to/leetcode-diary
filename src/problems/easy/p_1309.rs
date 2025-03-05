pub fn freq_alphabets(s: String) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut index = 0;

    while index <= n - 1 {
        let t_index = index + 2;
        if t_index > n - 1 {
            result.push((chars[index].to_digit(10).unwrap() as u8 + b'a' - 1) as char);
        } else {
            let target = &s[index..=t_index];
            if (target).contains('#') {
                result.push((target[0..2].parse::<u8>().unwrap() + b'a' - 1) as char);
                index = t_index
            } else {
                result.push((chars[index].to_digit(10).unwrap() as u8 + b'a' - 1) as char);
            }
        }
        index += 1;
    }
    result
}
/*
pub fn freq_alphabets(s: String) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut i = 0;
    while i < n {
        if i + 2 < n && chars[i + 2] == '#' {
            // 数字が2桁の場合 (10#, 11#, ..., 26#)
            let num: u8 = format!("{}{}", chars[i], chars[i+1]).parse().unwrap();
            result.push((num + b'a' - 1) as char);
            i += 3;
        } else {
            // 数字が1桁の場合 (1, 2, ..., 9)
            let num = chars[i].to_digit(10).unwrap() as u8;
            result.push((num + b'a' - 1) as char);
            i += 1;
        }
    }
    result
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("jkab".to_string(), freq_alphabets("10#11#12".to_string()));
        assert_eq!("acz".to_string(), freq_alphabets("1326#".to_string()));
    }
}
