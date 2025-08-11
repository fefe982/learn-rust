// https://leetcode.com/problems/continuous-subarrays/
// 2762. Continuous Subarrays
pub struct Solution;
impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut maxheap = std::collections::BinaryHeap::new();
        let mut minheap = std::collections::BinaryHeap::new();
        let mut i = 0;
        let mut j = 0;
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        while j < nums.len() {
            maxheap.push((nums[j], j));
            minheap.push(std::cmp::Reverse((nums[j], j)));
            max = max.max(nums[j]);
            min = min.min(nums[j]);
            while max - min > 2 {
                i += 1;
                while let Some(&(n, ii)) = maxheap.peek() {
                    if ii < i {
                        maxheap.pop();
                    } else {
                        max = n;
                        break;
                    }
                }
                while let Some(&std::cmp::Reverse((n, ii))) = minheap.peek() {
                    if ii < i {
                        minheap.pop();
                    } else {
                        min = n;
                        break;
                    }
                }
            }
            ans += (j - i + 1) as i64;
            j += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_continuous_subarrays() {
        assert_eq!(Solution::continuous_subarrays(vec![5, 4, 2, 4]), 8);
        assert_eq!(Solution::continuous_subarrays(vec![1, 2, 3]), 6);
    }
}
