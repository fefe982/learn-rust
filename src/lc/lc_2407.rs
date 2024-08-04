// https://leetcode.com/problems/longest-increasing-subsequence-ii/
// 2407. Longest Increasing Subsequence II
pub struct Solution;
impl Solution {
    fn put(m: &mut Vec<i32>, idx: usize, l: i32, r: i32, i: i32, v: i32) {
        if l + 1 == r {
            m[idx] = m[idx].max(v);
            return;
        }
        let mid = (l + r) / 2;
        if i < mid {
            Self::put(m, idx * 2 + 1, l, mid, i, v);
        } else {
            Self::put(m, idx * 2 + 2, mid, r, i, v);
        }
        m[idx] = m[idx * 2 + 1].max(m[idx * 2 + 2]);
    }
    fn get(m: &Vec<i32>, idx: usize, l: i32, r: i32, il: i32, ir: i32) -> i32 {
        if il <= l && r <= ir {
            return m[idx];
        }
        if r <= il || ir <= l {
            return 0;
        }
        let mid = (l + r) / 2;
        return Self::get(m, idx * 2 + 1, l, mid, il, ir).max(Self::get(m, idx * 2 + 2, mid, r, il, ir));
    }
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        let sz = 65536 * 2;
        let mut m = vec![0; sz as usize * 2];
        let mut res = 0;
        for i in 0..nums.len() {
            let next = Self::get(&m, 0, 0, sz, (nums[i] - k).max(0), nums[i]) + 1;
            res = res.max(next);
            Self::put(&mut m, 0, 0, sz, nums[i], next);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_lis() {
        assert_eq!(Solution::length_of_lis(vec![1, 100, 500, 100000, 100000], 100000), 4);
        assert_eq!(Solution::length_of_lis(vec![4, 2, 1, 4, 3, 4, 5, 8, 15], 3), 5);
        assert_eq!(Solution::length_of_lis(vec![7, 4, 5, 1, 8, 12, 4, 7], 5), 4);
        assert_eq!(Solution::length_of_lis(vec![1, 5], 1), 1);
    }
}
