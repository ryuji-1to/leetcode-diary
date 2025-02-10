use std::collections::HashMap;

pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut map = heights.iter().zip(names).collect::<Vec<_>>();
    map.sort_unstable_by_key(|(&v, _)| -v);
    let (_, sorted_name): (Vec<i32>, Vec<String>) = map.into_iter().unzip();
    sorted_name
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec!["Mary", "Emma", "John"],
            sort_people(
                vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
                vec![180, 165, 170]
            )
        );
        assert_eq!(
            vec!["Bob", "Alice", "Bob"],
            sort_people(
                vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()],
                vec![155, 185, 150]
            )
        );
    }
}
