// struct RecentCounter {
//     requests: Vec<i32>,
// }
//
// impl RecentCounter {
//     fn new() -> Self {
//         Self {
//             requests: Vec::new(),
//         }
//     }
//
//     fn ping(&mut self, t: i32) -> i32 {
//         self.requests.push(t);
//         let min_range = t - 3000;
//         self.requests
//             .iter()
//             .filter(|&&x| x >= min_range && x <= t)
//             .count() as i32
//     }
// }

use std::collections::VecDeque;

struct RecentCounter {
    requests: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self {
            requests: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.requests.push_back(t);

        while let Some(&front) = self.requests.front() {
            if front < t - 3000 {
                self.requests.pop_front();
            } else {
                break;
            }
        }

        self.requests.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut rc = RecentCounter::new();
        let ex = rc.ping(1);
        assert_eq!(1, ex);
        let ex = rc.ping(100);
        assert_eq!(2, ex);
        let ex = rc.ping(3001);
        assert_eq!(3, ex);
        let ex1 = rc.ping(3002);
        assert_eq!(3, ex);
    }
}
