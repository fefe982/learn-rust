// https://leetcode.com/problems/smallest-rotation-with-highest-score/
// 798. Smallest Rotation with Highest Score
pub struct Solution;
impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let mut chng = vec![1; nums.len()];
        let l = nums.len();
        for (i, n) in nums.into_iter().enumerate() {
            chng[(i + l + 1 - n as usize) % l] -= 1;
        }
        let m = chng[0];
        chng.into_iter()
            .enumerate()
            .fold((0, m, 0), |(maxi, max, sum), (i, n)| {
                let s = sum + n;
                if s > max {
                    (i, s, s)
                } else {
                    (maxi, max, s)
                }
            })
            .0 as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_best_rotation() {
        assert_eq!(Solution::best_rotation(vec![2, 3, 1, 4, 0]), 3);
        assert_eq!(Solution::best_rotation(vec![1, 3, 0, 2, 4]), 0);
    }
}
