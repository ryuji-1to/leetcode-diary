use std::collections::HashMap;

pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    // let filtered = arr
    //     .iter()
    //     .enumerate()
    //     .filter(|&(i, x)| !arr[..i].contains(x) && !arr[i + 1..].contains(x))
    //     .collect::<Vec<_>>();
    // filtered
    //     .get((k - 1) as usize)
    //     .cloned()
    //     .unwrap_or((1, &String::from("")))
    //     .1
    //     .to_string()
    let seen = arr.iter().fold(HashMap::new(), |mut acc, x| {
        acc.entry(x).and_modify(|v| *v = true).or_insert(false);
        acc
    });
    arr.iter()
        .filter(|&x| !*seen.get(x).unwrap())
        .nth((k - 1) as usize)
        .unwrap_or(&String::new())
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "a".to_string(),
            kth_distinct(
                vec![
                    "d".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string()
                ],
                2
            )
        );
        assert_eq!(
            "aaa".to_string(),
            kth_distinct(
                vec!["aaa".to_string(), "aa".to_string(), "a".to_string(),],
                1
            )
        );
        assert_eq!(
            "".to_string(),
            kth_distinct(vec!["a".to_string(), "b".to_string(), "a".to_string(),], 3)
        );
    }
}
