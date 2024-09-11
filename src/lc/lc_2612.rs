// https://leetcode.com/problems/minimum-reverse-operations/
// 2612. Minimum Reverse Operations
pub struct Solution;
impl Solution {
    pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
        let n = n as usize;
        let p = p as usize;
        let k = k as usize;
        let mut ans = vec![-1; n];
        ans[p] = 0;
        let banned = banned.into_iter().collect::<std::collections::HashSet<i32>>();
        let mut remain = vec![std::collections::BTreeSet::new(); 2];
        for i in 0..n {
            if i != p && !banned.contains(&(i as i32)) {
                remain[i & 1].insert(i);
            }
        }
        let mut q = vec![p];
        while !q.is_empty() {
            let mut nq = vec![];
            let mut last_len = 0;
            for c in q {
                let parity = (c + k - 1) & 1;
                let l = if c + 1 >= k { c + 1 - k } else { 0 };
                let r = c.min(n - k);
                for &n in remain[parity].range((l * 2 + k - 1 - c)..=(r * 2 + k - 1 - c)) {
                    nq.push(n);
                    ans[n] = ans[c] + 1;
                }
                for i in last_len..nq.len() {
                    remain[parity].remove(&nq[i]);
                }
                last_len = nq.len();
            }
            q = nq;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_reverse_operations() {
        assert_eq!(Solution::min_reverse_operations(5, 0, vec![], 2), [0, 1, 2, 3, 4]);
        assert_eq!(Solution::min_reverse_operations(4, 0, vec![1, 2], 4), [0, -1, -1, 1]);
        assert_eq!(
            Solution::min_reverse_operations(5, 0, vec![2, 4], 3),
            [0, -1, -1, -1, -1]
        );
    }
}
