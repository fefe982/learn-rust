// https://leetcode.com/problems/swap-for-longest-repeated-character-substring/
// 1156. Swap For Longest Repeated Character Substring
pub struct Solution;
impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let mut m = std::collections::HashMap::<u8, Vec<(usize, usize)>>::new();
        let mut last = 0;
        for (i, &c) in text.as_bytes().iter().enumerate() {
            if c == last {
                m.get_mut(&c).unwrap().last_mut().unwrap().1 += 1;
            } else {
                m.entry(c).or_default().push((i, 1));
            }
            last = c;
        }
        let mut max = 0;
        for (_, v) in m {
            for i in 0..v.len() {
                max = std::cmp::max(max, v[i].1 + if v.len() > 1 { 1 } else { 0 });
                if i > 0 && v[i - 1].0 + v[i - 1].1 + 1 == v[i].0 {
                    max = std::cmp::max(max, v[i - 1].1 + v[i].1 + if v.len() > 2 { 1 } else { 0 });
                }
            }
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_rep_opt1() {
        assert_eq!(Solution::max_rep_opt1(String::from("aaabbaaa")), 4);
        assert_eq!(Solution::max_rep_opt1(String::from("ababa")), 3);
        assert_eq!(Solution::max_rep_opt1(String::from("aaabaaa")), 6);
        assert_eq!(Solution::max_rep_opt1(String::from("aaaaa")), 5);
    }
}
