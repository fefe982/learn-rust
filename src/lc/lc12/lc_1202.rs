// https://leetcode.com/problems/smallest-string-with-swaps/
// 1202. Smallest String With Swaps
pub struct Solution;
impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut uf = (0..s.len()).collect::<Vec<_>>();
        fn find(uf: &mut Vec<usize>, x: usize) -> usize {
            if uf[x] != x {
                uf[x] = find(uf, uf[x]);
            }
            uf[x]
        }
        fn union(uf: &mut Vec<usize>, x: usize, y: usize) {
            let x = find(uf, x);
            let y = find(uf, y);
            uf[x] = y;
        }
        for p in pairs {
            union(&mut uf, p[0] as usize, p[1] as usize);
        }
        let mut map = std::collections::HashMap::new();
        for i in 0..s.len() {
            let root = find(&mut uf, i);
            map.entry(root).or_insert(vec![]).push(s[i]);
        }
        for (_, v) in map.iter_mut() {
            v.sort_unstable_by(|a, b| b.cmp(a));
        }
        for i in 0..s.len() {
            let root = find(&mut uf, i);
            let v = map.get_mut(&root).unwrap();
            s[i] = v.pop().unwrap();
        }
        s.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn smallest_string_with_swaps() {
        assert_eq!(
            Solution::smallest_string_with_swaps("dcab".to_string(), vec_vec![[0, 3], [1, 2]]),
            "bacd"
        );
        assert_eq!(
            Solution::smallest_string_with_swaps("dcab".to_string(), vec_vec![[0, 3], [1, 2], [0, 2]]),
            "abcd"
        );
        assert_eq!(
            Solution::smallest_string_with_swaps("cba".to_string(), vec_vec![[0, 1], [1, 2]]),
            "abc"
        );
    }
}
