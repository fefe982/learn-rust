// https://leetcode.com/problems/watering-plants/
// 2079. Watering Plants
pub struct Solution;
impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut cur = 0;
        let mut ans = 0;
        let n = plants.len();
        for (i, p) in plants.into_iter().enumerate() {
            if cur + p <= capacity {
                cur += p;
            } else {
                ans += i as i32 * 2;
                cur = p;
            }
        }
        ans + n as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_watering_plants() {
        assert_eq!(Solution::watering_plants(vec![2, 2, 3, 3], 5), 14);
        assert_eq!(Solution::watering_plants(vec![1, 1, 1, 4, 2, 3], 4), 30);
        assert_eq!(Solution::watering_plants(vec![7, 7, 7, 7, 7, 7, 7], 8), 49);
    }
}
