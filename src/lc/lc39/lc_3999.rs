// https://leetcode.com/problems/minimum-number-of-string-groups-through-transformations/
// 3999. Minimum Number of String Groups Through Transformations
pub struct Solution;
impl Solution {
    fn get_key(w: &Vec<u8>) -> Vec<u8> {
        let n = w.len();
        let mut i = 0;
        let mut j = 1;
        while j < n {
            let mut k = 0;
            while k < n && w[(i + k) % n] == w[(j + k) % n] {
                k += 1;
            }
            if k >= n {
                break;
            }
            if w[(i + k) % n] < w[(j + k) % n] {
                j += k + 1;
            } else {
                let nj = j.max(i + k) + 1;
                i = j;
                j = nj;
            }
        }
        let mut r = Vec::with_capacity(w.len());
        for k in 0..w.len() {
            r.push(w[(i + k) % n])
        }
        r
    }
    pub fn minimum_groups(words: Vec<String>) -> i32 {
        let mut h = std::collections::HashSet::new();
        for w in words {
            let mut e = vec![];
            let mut o = vec![];
            let wb = w.as_bytes();
            for i in 0..wb.len() {
                if i & 1 == 0 {
                    e.push(wb[i]);
                } else {
                    o.push(wb[i]);
                }
            }
            e = Self::get_key(&e);
            o = Self::get_key(&o);
            e.append(&mut o);
            h.insert(e);
        }
        h.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_groups() {
        assert_eq!(Solution::minimum_groups(vec_str!["ntgwz", "zwntg"]), 1);
        assert_eq!(
            Solution::minimum_groups(vec_str!["abc", "cab", "bac", "acb", "bca", "cba"]),
            3
        );
        assert_eq!(
            Solution::minimum_groups(vec_str!["leet", "abb", "bab", "deed", "edde", "code", "bba"]),
            5
        );
    }
}
