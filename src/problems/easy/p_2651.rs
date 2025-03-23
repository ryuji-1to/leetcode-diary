pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
    (arrival_time + delayed_time) % 24
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(20, find_delayed_arrival_time(15, 5));
        assert_eq!(0, find_delayed_arrival_time(13, 11));
    }
}
