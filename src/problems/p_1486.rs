pub fn xor_operation(n: i32, mut start: i32) -> i32 {
    // let mut result = start;
    // for i in 1..n {
    //     println!("{} , {} ", result, start + 2 * i);
    //     result = result ^ (start + 2 * i);
    // }
    // result
    (1..n).fold(start, |acc, v| acc ^ start + 2 * v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(8, xor_operation(5, 0));
        assert_eq!(8, xor_operation(4, 3));
    }
}
