pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
    let x_to_z = x.abs_diff(z);
    let y_to_z = y.abs_diff(z);
    match x_to_z.cmp(&y_to_z) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => 2,
        std::cmp::Ordering::Equal => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, find_closest(2, 7, 4));
        assert_eq!(2, find_closest(2, 5, 6));
        assert_eq!(0, find_closest(1, 5, 3));
    }
}
