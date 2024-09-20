// https://leetcode.com/problems/shortest-palindrome/
// 214. Shortest Palindrome
pub struct Solution;
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let mut ss = s.clone();
        ss.push('0');
        ss.extend(s.chars().rev());
        let bytes = ss.as_bytes();
        let mut z = Vec::with_capacity(bytes.len());
        let mut l = 0;
        let mut r = 0;
        z.push(0);
        for i in 1..bytes.len() {
            if i < r {
                z.push(z[i - l].min(r - i));
            } else {
                z.push(0);
            }
            while i + z[i] < bytes.len() && bytes[i + z[i]] == bytes[z[i]] {
                z[i] += 1;
            }
            if i + z[i] > r {
                l = i;
                r = i + z[i];
            }
            if z[i] + i == bytes.len() {
                return format!("{}{}", &ss[s.len() + 1..i], s);
            }
        }
        unreachable!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shortest_palindrome() {
        assert_eq!(Solution::shortest_palindrome(String::from("")), String::from(""));
        assert_eq!(
            Solution::shortest_palindrome(String::from("aacecaaa")),
            String::from("aaacecaaa")
        );
        assert_eq!(
            Solution::shortest_palindrome(String::from("abcd")),
            String::from("dcbabcd")
        );
    }
}
