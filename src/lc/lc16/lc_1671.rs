// https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/
// 1671. Minimum Number of Removals to Make Mountain Array
pub struct Solution;
impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let mut left = vec![];
        let mut map = std::collections::BTreeMap::new();
        map.insert(0, 0);
        for &n in &nums {
            let (_, &lv) = map.range(..n).rev().next().unwrap();
            left.push(lv);
            if let Some((&nk, &nv)) = map.range(n..).next() {
                if n < nk {
                    if nv == lv + 1 {
                        map.remove(&nk);
                    }
                    map.insert(n, lv + 1);
                }
            } else {
                map.insert(n, lv + 1);
            }
        }
        map.clear();
        map.insert(0, 0);
        let mut max = 0;
        for (i, &n) in nums.iter().enumerate().rev() {
            let (_, &lv) = map.range(..n).rev().next().unwrap();
            if lv > 0 && left[i] > 0 {
                max = max.max(lv + left[i] + 1);
            }
            if let Some((&nk, &nv)) = map.range(n..).next() {
                if n < nk {
                    if nv == lv + 1 {
                        map.remove(&nk);
                    }
                    map.insert(n, lv + 1);
                }
            } else {
                map.insert(n, lv + 1);
            }
        }
        (nums.len() - max) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_mountain_removals() {
        assert_eq!(Solution::minimum_mountain_removals(vec![1, 3, 1]), 0);
        assert_eq!(Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]), 3);
    }
}
