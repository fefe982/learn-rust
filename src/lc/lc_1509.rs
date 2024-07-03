// https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/
// 1509. Minimum Difference Between Largest and Smallest Value in Three Moves
pub struct Solution;
impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        if nums.len() <= 4 {
            return 0;
        }
        let mut max = vec![i32::MIN; 4];
        let mut min = vec![i32::MAX; 4];
        for num in nums {
            for i in 0..4 {
                if num > max[i] {
                    if i > 0 {
                        max[i - 1] = max[i];
                    }
                    max[i] = num;
                }
                if num < min[i] {
                    if i > 0 {
                        min[i - 1] = min[i];
                    }
                    min[i] = num;
                }
            }
        }
        let mut ans = i32::MAX;
        for (mx, mn) in max.into_iter().zip(min.into_iter().rev()) {
            ans = ans.min(mx - mn);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_difference() {
        assert_eq!(Solution::min_difference(vec![5, 3, 2, 4]), 0);
        assert_eq!(Solution::min_difference(vec![1, 5, 0, 10, 14]), 1);
        assert_eq!(Solution::min_difference(vec![3, 100, 20]), 0);
    }
}
