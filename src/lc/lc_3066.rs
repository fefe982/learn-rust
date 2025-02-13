// https://leetcode.cn/problems/minimum-operations-to-exceed-threshold-value-ii/
// 3066. Minimum Operations to Exceed Threshold Value II
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = std::collections::BinaryHeap::new();
        let k = k as u32;
        let mut min = k;
        let mut cnthigh = 0;
        for n in nums {
            let n = n as u32;
            if 3 * n < k {
                h.push(std::cmp::Reverse(n));
            } else if n < k {
                min = min.min(n);
                cnthigh += 1;
            }
        }
        let mut cnt = 0;
        while let Some(std::cmp::Reverse(n1)) = h.pop() {
            cnt += 1;
            if let Some(std::cmp::Reverse(n2)) = h.pop() {
                let s = n1 * 2 + n2;
                if 3 * s < k {
                    h.push(std::cmp::Reverse(s));
                } else if s < k {
                    min = min.min(s);
                    cnthigh += 1;
                }
            } else {
                if n1 * 2 + min >= k {
                    cnthigh -= 1;
                }
            }
        }
        cnt + (cnthigh + 1) / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(
            Solution::min_operations(vec![999999999, 999999999, 999999999], 1000000000),
            2
        );
        assert_eq!(Solution::min_operations(vec![2, 11, 10, 1, 3], 10), 2);
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 4, 9], 20), 4);
    }
}
