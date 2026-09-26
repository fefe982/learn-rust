// https://leetcode.com/problems/longest-subarray-divisible-by-k-with-at-most-one-negation-ii/
// 4064. Longest Subarray Divisible by K with At Most One Negation II
pub struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut sm = std::collections::HashMap::new();
        let mut em = std::collections::HashMap::new();
        let mut r = std::collections::HashMap::new();
        let mut ans = 0;
        let mut sum = 0;
        sm.insert(0, 0);
        em.insert(0, 0);
        for (i, &n) in nums.iter().enumerate() {
            let n = (n % k + k) % k;
            sum = (sum + n) % k;
            if let Some(&j) = sm.get(&sum) {
                ans = ans.max(i as i32 - j + 1);
            }
            sm.entry(sum).or_insert(i as i32 + 1);
            em.insert(sum, i as i32 + 1);
            r.entry((n * 2) % k).or_insert(vec![]).push(i as i32 + 1);
        }
        let mut sa = sm.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();
        sa.sort_by_key(|&(_, v)| v);
        for (r2, vr) in r {
            if r2 == 0 {
                continue;
            }
            let mut i = 0;
            let mut j = 0;
            while i < vr.len() && j < sa.len() {
                if sa[j].1 < vr[i] {
                    if let Some(&j2) = em.get(&((sa[j].0 + r2) % k)) {
                        if j2 >= vr[i] {
                            ans = ans.max(j2 - sa[j].1);
                        }
                    }
                    j += 1;
                } else {
                    i += 1;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_subarray() {
        assert_eq!(Solution::longest_subarray(vec![49, -6, -35, -35, 25, 26, 42], 5), 6);
        assert_eq!(Solution::longest_subarray(vec![4, -13], 6), 0);
        assert_eq!(Solution::longest_subarray(vec![9, 13, 10], 5), 1);
        assert_eq!(Solution::longest_subarray(vec![4, 1, 2], 3), 3);
        assert_eq!(Solution::longest_subarray(vec![5, 3, 4], 7), 2);
        assert_eq!(Solution::longest_subarray(vec![2, 2, 5], 6), 2);
    }
}
