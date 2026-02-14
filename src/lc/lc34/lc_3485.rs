// https://leetcode.com/problems/longest-common-prefix-of-k-strings-after-removal/
// 3485. Longest Common Prefix of K Strings After Removals
pub struct Solution;
impl Solution {
    pub fn longest_common_prefix(words: Vec<String>, k: i32) -> Vec<i32> {
        let mut idx = (0..words.len()).collect::<Vec<_>>();
        let words = words.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        idx.sort_by_key(|&i| words[i]);
        let mut mx = 0;
        let mut mx2 = 0;
        let mut imx = 0;
        let k = k as usize;
        for i in k..=idx.len() {
            let l = words[idx[i - k]];
            let r = words[idx[i - 1]];
            let mut j = 0;
            while j < l.len() && j < r.len() && l[j] == r[j] {
                j += 1;
            }
            if j >= mx {
                mx2 = mx;
                mx = j;
                imx = i - k;
            } else if j > mx2 {
                mx2 = j;
            }
        }
        let mut ans = vec![0; words.len()];
        for i in 0..idx.len() {
            if i >= imx && i < imx + k {
                ans[idx[i]] = mx2 as i32;
            } else {
                ans[idx[i]] = mx as i32;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec_str!["fabff", "eaae", "fdeea"], 1),
            vec![5, 5, 5]
        );
        assert_eq!(
            Solution::longest_common_prefix(vec_str!["jump", "run", "run", "jump", "run"], 2),
            vec![3, 4, 4, 3, 4]
        );
        assert_eq!(
            Solution::longest_common_prefix(vec_str!["dog", "racer", "car"], 2),
            vec![0, 0, 0]
        );
    }
}
