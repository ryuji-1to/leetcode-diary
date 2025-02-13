pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
    let mut result = vec![pref[0]];
    let mut tmp = pref[0];
    for &v in &pref[1..] {
        let _v = tmp ^ v;
        tmp = tmp ^ _v;
        result.push(_v);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![5, 7, 2, 3, 2], find_array(vec![5, 2, 0, 3, 1]));
        assert_eq!(vec![13], find_array(vec![13]));
    }
}
