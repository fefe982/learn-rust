// https://leetcode.com/problems/sort-characters-by-frequency/
// 451. Sort Characters By Frequency
pub struct Solution;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut m = std::collections::HashMap::<char, i32>::new();
        for c in s.chars() {
            *m.entry(c).or_default() += 1;
        }
        let mut v = m.into_iter().collect::<Vec<_>>();
        v.sort_unstable_by_key(|x| std::cmp::Reverse(x.1));
        v.iter().fold(String::new(), |mut acc, x| {
            for _ in 0..x.1 {
                acc.push(x.0);
            }
            acc
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(s: &str, result: &str) {
        assert!(s.len() == result.len());
        let mut counts = std::collections::HashMap::<char, i32>::new();
        for c in s.chars() {
            *counts.entry(c).or_default() += 1;
        }
        let mut lastc = ' ';
        let mut last_count = i32::MAX;
        let mut count = i32::MAX;
        for c in result.chars() {
            if c != lastc {
                assert!(count <= last_count);
                lastc = c;
                last_count = count;
                count = 0;
            } else {
                count += 1;
                let cc = counts.get_mut(&c);
                assert!(cc.is_some());
                let cc = cc.unwrap();
                *cc -= 1;
                assert!(*cc >= 0);
            }
        }
        assert!(count <= last_count);
    }
    #[test]
    fn test_frequnce_sort() {
        check("tree", &Solution::frequency_sort(String::from("tree")));
        check("cccaaa", &Solution::frequency_sort(String::from("cccaaa")));
        check("Aabb", &Solution::frequency_sort(String::from("Aabb")));
    }
}
