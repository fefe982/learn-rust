// https://leetcode.com/problems/sort-vowels-in-a-string/
// 2785. Sort Vowels in a String
pub struct Solution;
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut c = s.chars().collect::<Vec<char>>();
        let v = c
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| {
                if x == 'a'
                    || x == 'e'
                    || x == 'i'
                    || x == 'o'
                    || x == 'u'
                    || x == 'A'
                    || x == 'E'
                    || x == 'I'
                    || x == 'O'
                    || x == 'U'
                {
                    Some((x, i))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if v.is_empty() {
            return s;
        }
        let vi = v.iter().map(|&(_, i)| i).collect::<Vec<_>>();
        let mut v = v.into_iter().map(|(x, _)| x).collect::<Vec<_>>();
        v.sort();
        for (x, i) in v.into_iter().zip(vi.into_iter()) {
            c[i] = x;
        }
        c.into_iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_vowels() {
        assert_eq!(Solution::sort_vowels("lEetcOde".to_string()), "lEOtcede".to_string());
        assert_eq!(Solution::sort_vowels("lYmpH".to_string()), "lYmpH".to_string());
    }
}
