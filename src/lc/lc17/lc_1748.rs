// https://leetcode.com/problems/sum-of-unique-elements/
// 1748. Sum of Unique Elements
pub struct Solution;
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut cnt = vec![0; 101];
        for n in nums {
            cnt[n as usize] += 1;
        }
        for i in 1..=100 {
            if cnt[i] == 1 {
                ans += i as i32;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_unique() {
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
        assert_eq!(Solution::sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
    }
}
