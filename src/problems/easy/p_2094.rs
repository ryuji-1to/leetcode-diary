pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    // let mut result = std::collections::HashSet::new();
    // for i in 0..digits.len() {
    //     if digits[i] == 0 {
    //         continue;
    //     }
    //     for j in 0..digits.len() {
    //         if i == j {
    //             continue;
    //         }
    //         for k in 0..digits.len() {
    //             if i == k || j == k || digits[k] % 2 != 0 {
    //                 continue;
    //             }
    //             let n = digits[i] * 100 + digits[j] * 10 + digits[k];
    //             if n % 2 == 0 {
    //                 result.insert(n as i32);
    //             }
    //         }
    //     }
    // }
    // let mut result = result.iter().map(|&c| c).collect::<Vec<i32>>();
    // result.sort_unstable();
    // result

    let n = digits.len();
    let even_digits: Vec<_> = (0..n).filter(|&i| digits[i] % 2 == 0).collect();
    let non_zero_digits: Vec<_> = (0..n).filter(|&i| digits[i] != 0).collect();
    if even_digits.is_empty() || non_zero_digits.is_empty() {
        return vec![];
    }
     let mut results = std::collections::BTreeSet::new();
    
    // 最初の桁は非ゼロから
    for &i in &non_zero_digits {
        // 2番目の桁は任意（ただし最初と異なる）
        for j in 0..n {
            if i == j { continue; }
            
            // 3番目の桁は偶数から（ただし最初・2番目と異なる）
            for &k in &even_digits {
                if k == i || k == j { continue; }
                
                let number = digits[i] * 100 + digits[j] * 10 + digits[k];
                results.insert(number);
            }
        }
    }
    
    results.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320],
            find_even_numbers(vec![2, 1, 3, 0])
        );

        assert_eq!(
            vec![222, 228, 282, 288, 822, 828, 882],
            find_even_numbers(vec![2, 2, 8, 8, 2])
        );

        assert_eq!(vec![] as Vec<i32>, find_even_numbers(vec![3, 7, 5]));
    }
}
