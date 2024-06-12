// https://leetcode.com/problems/maximum-fruits-harvested-after-at-most-k-steps/
// 2106. Maximum Fruits Harvested After at Most K Steps
pub struct Solution;
impl Solution {
    fn check(l: i32, p: i32, r: i32, k: i32) -> bool {
        return r <= p || (l >= p && r <= p + k) || (l <= p && r >= p && r - l + std::cmp::min(p - l, r - p) <= k);
    }
    pub fn max_total_fruits(mut fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        fruits.push(vec![i32::MAX, 0]);
        let mut max = 0;
        let mut l = fruits.partition_point(|x| start_pos - x[0] > k);
        let mut r = l;
        let mut sum = 0;
        while l <= fruits.len() && r < fruits.len() && fruits[r][0] <= start_pos + k {
            while r < fruits.len() && Self::check(fruits[l][0], start_pos, fruits[r][0], k) {
                sum += fruits[r][1];
                r += 1;
            }
            max = std::cmp::max(max, sum);
            sum -= fruits[l][1];
            l += 1;
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_total_fruits() {
        assert_eq!(Solution::max_total_fruits(vec_vec![[2, 8], [6, 3], [8, 6]], 5, 4), 9);
        assert_eq!(
            Solution::max_total_fruits(vec_vec![[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]], 5, 4),
            14
        );
        assert_eq!(Solution::max_total_fruits(vec_vec![[0, 3], [6, 4], [8, 5]], 3, 2), 0);
    }
}
