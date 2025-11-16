// https://leetcode.cn/problems/shu-zu-zhong-de-ni-xu-dui-lcof/
// LCR 170. 交易逆序对的总数
pub struct Solution;
impl Solution {
    fn merge(nums: &mut Vec<i32>, buf: &mut Vec<i32>, l: usize, r: usize) -> i32 {
        if l + 1 == r {
            return 0;
        }
        let m = (l + r) >> 1;
        let mut ans = Self::merge(nums, buf, l, m) + Self::merge(nums, buf, m, r);
        let mut i = l;
        let mut j = m;
        let mut k = l;
        while i < m && j < r {
            if nums[i] <= nums[j] {
                buf[k] = nums[i];
                i += 1;
            } else {
                buf[k] = nums[j];
                ans += (m - i) as i32;
                j += 1;
            }
            k += 1;
        }
        while i < m {
            buf[k] = nums[i];
            i += 1;
            k += 1;
        }
        while j < r {
            buf[k] = nums[j];
            j += 1;
            k += 1;
        }
        for i in l..r {
            nums[i] = buf[i];
        }
        ans
    }
    pub fn reverse_pairs(record: Vec<i32>) -> i32 {
        if record.is_empty() {
            return 0;
        }
        let len = record.len();
        let mut record = record;
        Self::merge(&mut record, &mut vec![0; len], 0, len)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::reverse_pairs(vec![1, 2, 1, 2, 1]), 3);
        assert_eq!(Solution::reverse_pairs(vec![9, 7, 5, 4, 6]), 8);
    }
}
