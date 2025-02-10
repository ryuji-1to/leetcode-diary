use std::collections::HashMap;

struct ParkingSystem {
    map: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            map: HashMap::<i32, i32>::from([(1, big), (2, medium), (3, small)]),
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let result = match self.map.get(&car_type) {
            Some(&v) if v > 0 => {
                self.map.insert(car_type, v - 1);
                true
            }
            // {
            //     if v > 0 {
            //         self.map.insert(car_type, v - 1);
            //         true
            //     } else {
            //         false
            //     }
            // }
            // None => false,
            _ => false,
        };
        result
    }
}

// Your ParkingSystem object will be instantiated and called as such:
// let obj = ParkingSystem::new(big, medium, small);
// let ret_1: bool = obj.add_car(carType);
