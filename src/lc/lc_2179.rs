// https://leetcode.com/problems/count-good-triplets-in-an-array/
// 2179. Count Good Triplets in an Array
pub struct Solution;
impl Solution {
    fn get_cnt(cnt: &Vec<i32>, mut i: usize) -> i32 {
        let mut res = cnt[i];
        loop {
            i = i & (i + 1);
            if i == 0 {
                break;
            }
            i -= 1;
            res += cnt[i];
        }
        res
    }
    fn set_cnt(cnt: &mut Vec<i32>, mut i: usize) {
        while i < cnt.len() {
            cnt[i] += 1;
            i = i | (i + 1);
        }
    }
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let mut cnt = vec![0; n];
        let mut m2 = std::collections::HashMap::<i32, usize>::new();
        for (i2, n2) in nums2.into_iter().enumerate() {
            m2.insert(n2, i2);
        }
        let mut ans = 0;
        for (i1, n1) in nums1.into_iter().enumerate() {
            let i2 = m2[&n1];
            let before = Self::get_cnt(&cnt, i2);
            let after = n as i32 - i1 as i32 - i2 as i32 - 1 + before;
            ans += before as i64 * after as i64;
            Self::set_cnt(&mut cnt, i2);
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
