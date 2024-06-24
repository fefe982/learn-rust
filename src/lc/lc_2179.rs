// https://leetcode.com/problems/count-good-triplets-in-an-array/
// 2179. Count Good Triplets in an Array
pub struct Solution;
impl Solution {
    fn get_cnt(cnt: &Vec<i32>, i: usize, rl: usize, rr: usize, l: usize, r: usize) -> i32 {
        if l >= r {
            return 0;
        }
        if l <= rl && rr <= r {
            return cnt[i];
        }
        if r <= rl || rr <= l {
            return 0;
        }
        let mid = (rl + rr) / 2;
        Self::get_cnt(cnt, i * 2 + 1, rl, mid, l, r) + Self::get_cnt(cnt, i * 2 + 2, mid, rr, l, r)
    }
    fn set_cnt(cnt: &mut Vec<i32>, i: usize, rl: usize, rr: usize, idx: usize) {
        cnt[i] += 1;
        if rl + 1 == rr {
            return;
        }
        let mid = (rl + rr) / 2;
        if idx < mid {
            Self::set_cnt(cnt, i * 2 + 1, rl, mid, idx);
        } else {
            Self::set_cnt(cnt, i * 2 + 2, mid, rr, idx);
        }
    }
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let mut nn = 1;
        while nn < n {
            nn *= 2;
        }
        let mut cnt = vec![0; nn * 2];
        let mut m2 = std::collections::HashMap::<i32, usize>::new();
        for (i2, n2) in nums2.into_iter().enumerate() {
            m2.insert(n2, i2);
        }
        let mut ans = 0;
        for (i1, n1) in nums1.into_iter().enumerate() {
            let i2 = m2[&n1];
            let before = Self::get_cnt(&cnt, 0, 0, nn, 0, i2);
            let after = n as i32 - i1 as i32 - i2 as i32 - 1 + before;
            ans += before as i64 * after as i64;
            Self::set_cnt(&mut cnt, 0, 0, nn, i2);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_good_triplets() {
        assert_eq!(Solution::good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3]), 1);
        assert_eq!(Solution::good_triplets(vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3]), 4);
    }
}
