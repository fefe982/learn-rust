// https://leetcode.com/problems/count-paths-that-can-form-a-palindrome-in-a-tree/
// 2791. Count Paths That Can Form a Palindrome in a Tree
pub struct Solution;
impl Solution {
    pub fn count_palindrome_paths(parent: Vec<i32>, s: String) -> i64 {
        let mut g = vec![vec![]; parent.len()];
        let s = s.as_bytes();
        for i in 1..parent.len() {
            g[parent[i] as usize].push(i);
        }
        let mut cnt = std::collections::HashMap::<i32, i32>::new();
        let mut q = vec![(0, 0)];
        let mut res = 0;
        while let Some((i, mask)) = q.pop() {
            res += cnt.get(&mask).copied().unwrap_or_default() as i64;
            for j in 0..26 {
                res += cnt.get(&(mask ^ (1 << j))).copied().unwrap_or_default() as i64;
            }
            *cnt.entry(mask).or_default() += 1;
            for &c in &g[i] {
                q.push((c, mask ^ (1 << (s[c] - b'a'))));
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_palindrome_paths() {
        assert_eq!(
            Solution::count_palindrome_paths(vec![-1, 0, 0, 1, 1, 2], "acaabc".to_string()),
            8
        );
        assert_eq!(
            Solution::count_palindrome_paths(vec![-1, 0, 0, 0, 0], "aaaaa".to_string()),
            10
        );
    }
}
