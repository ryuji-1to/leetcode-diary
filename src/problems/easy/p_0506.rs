use std::collections::HashMap;

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut _score = score.clone();
    _score.sort_unstable_by(|a, b| b.cmp(a));
    let map = _score
        .iter()
        .enumerate()
        .map(|(i, x)| (x, i + 1))
        .collect::<HashMap<_, _>>();
    score
        .iter()
        .map(|x| {
            let place = *map.get(x).unwrap();
            match place {
                1 => "Gold Medal".to_string(),
                2 => "Silver Medal".to_string(),
                3 => "Bronze Medal".to_string(),
                x => x.to_string(),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![
                "Gold Medal".to_string(),
                "Silver Medal".to_string(),
                "Bronze Medal".to_string(),
                "4".to_string(),
                "5".to_string()
            ],
            find_relative_ranks(vec![5, 4, 3, 2, 1])
        );
        assert_eq!(
            vec![
                "Gold Medal".to_string(),
                "5".to_string(),
                "Bronze Medal".to_string(),
                "Silver Medal".to_string(),
                "4".to_string(),
            ],
            find_relative_ranks(vec![10, 3, 8, 9, 4])
        );
    }
}
