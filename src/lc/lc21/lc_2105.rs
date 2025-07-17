// https://leetcode.com/problems/watering-plants-ii/
// 2105. Watering Plants II
pub struct Solution;
impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let mut ans = 0;
        let mut a = capacity_a;
        let mut b = capacity_b;
        for i in 0..plants.len() / 2 {
            a -= plants[i];
            if a < 0 {
                ans += 1;
                a = capacity_a - plants[i];
            }
            b -= plants[plants.len() - i - 1];
            if b < 0 {
                ans += 1;
                b = capacity_b - plants[plants.len() - i - 1];
            }
        }
        if plants.len() % 2 == 1 && a.max(b) < plants[plants.len() / 2] {
            ans += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_refill() {
        assert_eq!(Solution::minimum_refill(vec![2, 2, 3, 3], 5, 5), 1);
        assert_eq!(Solution::minimum_refill(vec![2, 2, 3, 3], 3, 4), 2);
        assert_eq!(Solution::minimum_refill(vec![5], 10, 8), 0);
    }
}
