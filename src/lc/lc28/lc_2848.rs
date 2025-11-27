// https://leetcode.com/problems/points-that-intersect-with-cars/
// 2848. Points That Intersect With Cars
pub struct Solution;
impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut e = 0;
        let mut ans = 0;
        for n in nums {
            if n[0] > e {
                ans += n[1] - n[0] + 1;
                e = n[1];
            } else {
                let ne = e.max(n[1]);
                ans += ne - e;
                e = ne;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn number_of_points() {
        assert_eq!(Solution::number_of_points(vec_vec![[3, 6], [1, 5], [4, 7]]), 7);
        assert_eq!(Solution::number_of_points(vec_vec![[1, 3], [5, 8]]), 7);
    }
}
