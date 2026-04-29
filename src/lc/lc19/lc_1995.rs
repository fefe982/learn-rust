// https://leetcode.com/problems/count-special-quadruplets/
// 1995. Count Special Quadruplets
pub struct Solution;
impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        for a in 0..nums.len() {
            for b in a + 1..nums.len() {
                for c in b + 1..nums.len() {
                    for d in c + 1..nums.len() {
                        if nums[a] + nums[b] + nums[c] == nums[d] {
                            cnt += 1;
                        }
                    }
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_special_quadruplets() {
        assert_eq!(Solution::count_quadruplets(vec![1, 2, 3, 6]), 1);
        assert_eq!(Solution::count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
        assert_eq!(Solution::count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
    }
}
