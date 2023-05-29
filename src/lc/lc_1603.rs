// https://leetcode.com/problems/design-parking-system/
// 1603. Design Parking System
struct ParkingSystem {
    slot: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            slot: vec![big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if car_type < 1 || car_type > 3 {
            false
        } else if self.slot[car_type as usize - 1] > 0 {
            self.slot[car_type as usize - 1] -= 1;
            true
        } else {
            false
        }
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parking() {
        let mut obj = ParkingSystem::new(1, 2, 0);
        assert_eq!(obj.add_car(1), true);
        assert_eq!(obj.add_car(2), true);
        assert_eq!(obj.add_car(3), false);
        assert_eq!(obj.add_car(1), false);
    }
}
