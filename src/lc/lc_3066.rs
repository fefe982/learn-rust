// https://leetcode.cn/problems/minimum-operations-to-exceed-threshold-value-ii/
// 3066. Minimum Operations to Exceed Threshold Value II
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = std::collections::BinaryHeap::new();
        for n in nums {
            if n < k {
                h.push(std::cmp::Reverse(n as i64));
            }
        }
        let mut cnt = 0;
        let k = k as i64;
        loop {
            if let Some(std::cmp::Reverse(n1)) = h.pop() {
                cnt += 1;
                if let Some(std::cmp::Reverse(n2)) = h.pop() {
                    let s = n1 * 2 + n2;
                    if s < k {
                        h.push(std::cmp::Reverse(s));
                    }
                } else {
                    return cnt;
                }
            } else {
                return cnt;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![2, 11, 10, 1, 3], 10), 2);
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 20), 4);
    }
}
