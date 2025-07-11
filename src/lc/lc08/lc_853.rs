// https://leetcode.com/problems/car-fleet/
// 853. Car Fleet
pub struct Solution;
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars = position
            .into_iter()
            .zip(speed.into_iter())
            .map(|(p, s)| (p, (target - p) as f64 / s as f64))
            .collect::<Vec<_>>();
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let mut ans = 0;
        let mut time = 0.0;
        for (_, t) in cars {
            if t > time {
                time = t;
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn car_fleet() {
        assert_eq!(Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
        assert_eq!(Solution::car_fleet(10, vec![0], vec![10]), 1);
    }
}
