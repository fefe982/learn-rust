// https://leetcode.com/problems/maximize-the-minimum-game-score/
// 3449. Maximize the Minimum Game Score
pub struct Solution;
impl Solution {
    fn count(points: &Vec<i32>, v: i64, m: i64) -> bool {
        let mut count = 0;
        let mut count_last = 0;
        for i in 0..points.len() - 1 {
            count += 1;
            count_last += 1;
            let p = points[i] as i64;
            let nc = ((v - p * count_last + p - 1) / p).max(0);
            count += nc * 2;
            if count > m {
                return false;
            }
            count_last = nc;
        }
        let p = points[points.len() - 1] as i64;
        let nc = ((v - p * count_last + p - 1) / p).max(0);
        if nc > 0 {
            count += nc * 2 - 1;
        }
        count <= m
    }
    pub fn max_score(points: Vec<i32>, m: i32) -> i64 {
        let (minp, maxp) = points
            .iter()
            .fold((i32::MAX, i32::MIN), |(minp, maxp), &x| (minp.min(x), maxp.max(x)));
        let m = m as i64;
        let (mut left, mut right) = (minp as i64, maxp as i64 * m);
        if !Self::count(&points, left, m) {
            return 0;
        }
        if Self::count(&points, right, m) {
            return right;
        }
        while left + 1 < right {
            let mid = (left + right) / 2;
            if Self::count(&points, mid, m) {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_score() {
        assert_eq!(Solution::max_score(vec![1, 8], 10), 5);
        assert_eq!(Solution::max_score(vec![2, 4], 3), 4);
        assert_eq!(Solution::max_score(vec![1, 2, 3], 5), 2);
    }
}
