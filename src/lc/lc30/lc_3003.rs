// https://leetcode.com/problems/maximize-the-number-of-partitions-after-operations/
// 3003. Maximize the Number of Partitions After Operations
pub struct Solution;
impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        if k == 26 {
            return 1;
        }
        let s = s.as_bytes();
        let len = s.len();
        let mut seg = vec![(1, 0); len + 1];
        let mut mask: i32 = 0;
        let mut cseg = 1;
        let k = k as u32;
        for i in (0..len).rev() {
            let imask = 1 << (s[i] - b'a');
            mask |= imask;
            if mask.count_ones() > k {
                cseg += 1;
                mask = imask;
            }
            seg[i] = (cseg, mask);
        }
        mask = 0;
        let mut res = 0;
        cseg = 1;
        for i in 0..len {
            let (rseg, rmask) = seg[i + 1];
            let mut tseg = cseg + rseg;
            let sz = (mask | rmask).count_ones();
            if sz < k {
                tseg -= 1;
            } else if sz < 26 && mask.count_ones() == k && rmask.count_ones() == k {
                tseg += 1;
            }
            res = res.max(tseg);
            let imask = 1 << (s[i] - b'a');
            mask |= imask;
            if mask.count_ones() > k {
                cseg += 1;
                mask = imask;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_partitions_after_operations() {
        assert_eq!(Solution::max_partitions_after_operations("acbabb".to_string(), 1), 6);
        assert_eq!(Solution::max_partitions_after_operations("c".to_string(), 3), 1);
        assert_eq!(Solution::max_partitions_after_operations("accca".to_string(), 2), 3);
        assert_eq!(Solution::max_partitions_after_operations("aabaab".to_string(), 3), 1);
        assert_eq!(Solution::max_partitions_after_operations("xxyz".to_string(), 1), 4);
    }
}
