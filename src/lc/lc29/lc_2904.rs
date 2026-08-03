// https://leetcode.com/problems/shortest-and-lexicographically-smallest-beautiful-string/
// 2904. Shortest and Lexicographically Smallest Beautiful String
pub struct Solution;
impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let s = s.as_bytes();
        let mut pos = 0;
        let mut b = 0;
        let mut e = 0;
        let mut c = 0;
        let mut l = usize::MAX;
        let k = k as usize;
        while e < s.len() {
            if s[e] == b'1' {
                c += 1;
            }
            e += 1;
            while c > k {
                if s[b] == b'1' {
                    c -= 1;
                }
                b += 1;
            }
            while c == k && s[b] == b'0' {
                b += 1;
            }
            if c == k {
                if e - b < l {
                    l = e - b;
                    pos = b;
                } else if e - b == l && s[b..e] < s[pos..pos + l] {
                    pos = b;
                }
            }
        }
        if l == usize::MAX {
            return "".to_string();
        }
        s[pos..pos + l].iter().map(|&x| x as char).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shortest_beautiful_substring() {
        assert_eq!(
            Solution::shortest_beautiful_substring("100011001".to_string(), 3),
            "11001"
        );
        assert_eq!(Solution::shortest_beautiful_substring("1011".to_string(), 2), "11");
        assert_eq!(Solution::shortest_beautiful_substring("000".to_string(), 2), "");
    }
}
