// https://leetcode.com/problems/minimum-time-to-repair-cars/
// 2594. Minimum Time to Repair Cars
pub struct Solution;
impl Solution {
    fn check_cars(ranks: &Vec<i32>, cars: i64, t: i64) -> bool {
        let mut n = 0;
        let t = t as f64;
        for &r in ranks {
            n += (t / r as f64).sqrt() as i64;
        }
        return n >= cars;
    }
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut min_rank = i32::MAX;
        for &r in &ranks {
            if r < min_rank {
                min_rank = r;
            }
        }
        let min_rank = min_rank as i64;
        let cars = cars as i64;
        let mut high = min_rank * cars * cars;
        if ranks.len() == 1 {
            return high;
        }
        let mut low = high / ranks.len() as i64 / ranks.len() as i64;
        if Self::check_cars(&ranks, cars, low) {
            return low;
        }
        while low < high {
            if low + 1 == high {
                return high;
            }
            let mid = (low + high) / 2;
            if Self::check_cars(&ranks, cars, mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repair_cars() {
        assert_eq!(Solution::repair_cars(vec![4, 2, 3, 1], 10), 16);
        assert_eq!(Solution::repair_cars(vec![5, 1, 8], 6), 16);
    }
}
