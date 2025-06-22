// https://leetcode.com/problems/heaters/
// 475. Heaters
pub struct Solution;
impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut i = 0;
        let mut houses = houses;
        houses.sort_unstable();
        let mut heaters = heaters;
        heaters.sort_unstable();
        for h in houses {
            if h <= heaters[i] {
                r = r.max(heaters[i] - h);
            } else {
                while i + 1 < heaters.len() && heaters[i + 1] < h {
                    i += 1;
                }
                if i + 1 < heaters.len() && heaters[i + 1] - h < h - heaters[i] {
                    i += 1;
                    r = r.max(heaters[i] - h);
                } else {
                    r = r.max(h - heaters[i]);
                }
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_radius() {
        assert_eq!(Solution::find_radius(vec![1, 2, 3], vec![2]), 1);
        assert_eq!(Solution::find_radius(vec![1, 2, 3, 4], vec![1, 4]), 1);
        assert_eq!(Solution::find_radius(vec![1, 5], vec![2]), 3);
    }
}
