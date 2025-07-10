// https://leetcode.com/problems/find-the-number-of-good-pairs-ii/
// 3164. Find the Number of Good Pairs II
pub struct Solution;
impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let get_cnt = |v: Vec<i32>| -> std::collections::HashMap<i32, i32> {
            let mut cnt = std::collections::HashMap::<i32, i32>::new();
            for i in v {
                *cnt.entry(i).or_default() += 1;
            }
            cnt
        };
        let cnt1 = get_cnt(nums1);
        let cnt2 = get_cnt(nums2);
        let mut cntp = std::collections::HashMap::<i32, i32>::new();
        for (n, c) in cnt1 {
            if n % k != 0 {
                continue;
            }
            let n = n / k;
            let mut i = 1;
            while i * i < n {
                if n % i == 0 {
                    *cntp.entry(i).or_default() += c;
                    *cntp.entry(n / i).or_default() += c;
                }
                i += 1;
            }
            if i * i == n {
                *cntp.entry(i).or_default() += c;
            }
        }
        let mut res = 0;
        for (n, c) in cnt2 {
            if let Some(&c1) = cntp.get(&n) {
                res += c as i64 * c1 as i64;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_pairs() {
        assert_eq!(Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1), 5);
        assert_eq!(Solution::number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3), 2);
    }
}
