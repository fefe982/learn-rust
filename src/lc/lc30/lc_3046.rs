// https://leetcode.cn/problems/split-the-array/
// 3046. Split the Array
pub struct Solution;
impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut cnt = [0; 101];
        for n in nums {
            cnt[n as usize] += 1;
            if cnt[n as usize] > 2 {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_possible_to_split() {
        assert_eq!(Solution::is_possible_to_split(vec![1, 1, 2, 2, 3, 4]), true);
        assert_eq!(Solution::is_possible_to_split(vec![1, 1, 1, 1]), false);
    }
}
