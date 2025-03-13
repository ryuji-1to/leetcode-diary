/*
pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();
    if k == 0 {
        return vec![0; n];
    }
    if k > 0 {
        code.iter()
            .enumerate()
            .map(|(i, x)| {
                let mut result = 0;
                for j in (i + 1)..(i + 1 + k as usize) {
                    if j > n - 1 {
                        result += code[j % n];
                    } else {
                        result += code[j];
                    }
                }
                result
            })
            .collect()
    } else {
        code.iter()
            .enumerate()
            .map(|(i, x)| {
                let mut result = 0;
                for j in 1..((1 + k.abs()) as usize) {
                    let diff = i as i32 - j as i32;
                    if diff < 0 {
                        result += code[(n as i32 + diff) as usize]
                    } else {
                        result += code[diff as usize];
                    }
                }
                result
            })
            .collect()
    }
}
*/
pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();
    if k == 0 {
        return vec![0; n];
    }
    let mut result = vec![0; n];
    for i in 0..n {
        let mut sum = 0;
        if k > 0 {
            for j in 1..=k as usize {
                let idx = (i + j) % n;
                sum += code[idx];
            }
        } else {
            for j in 1..=(-k) as usize {
                let idx = (i + n - j) % n;
                sum += code[idx];
            }
        }
        result[i] = sum;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![12, 10, 16, 13], decrypt(vec![5, 7, 1, 4], 3));
        assert_eq!(vec![0, 0, 0, 0], decrypt(vec![1, 2, 3, 4], 0));
        assert_eq!(vec![12, 5, 6, 13], decrypt(vec![2, 4, 9, 3], -2));
    }
}
