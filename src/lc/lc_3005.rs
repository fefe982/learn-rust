// https://leetcode.com/problems/count-elements-with-maximum-frequency/
// 3005. Count Elements With Maximum Frequency
pub struct Solution;
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 101];
        for num in nums {
            cnt[num as usize] += 1;
        }
        cnt.sort_unstable();
        let pos = cnt.partition_point(|&x| x < cnt[100]);
        (101 - pos) as i32 * cnt[100]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_frequency_elements() {
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
    }
}
