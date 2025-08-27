// https://leetcode.com/problems/longest-chunked-palindrome-decomposition/
// 1147. Longest Chunked Palindrome Decomposition
pub struct Solution;
impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let text = text.as_bytes();
        let mut beg = 0;
        let mut end = text.len();
        let mut cnt = 0;
        let mut len = 1;
        loop {
            if beg + len > end - len {
                break;
            }
            if beg + len <= end - len && text[beg..beg + len] == text[end - len..end] {
                cnt += 2;
                beg += len;
                end -= len;
                len = 1;
            } else {
                len += 1;
            }
        }
        if beg != end {
            cnt += 1;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_decomposition() {
        assert_eq!(
            Solution::longest_decomposition(String::from("ghiabcdefhelloadamhelloabcdefghi")),
            7
        );
        assert_eq!(Solution::longest_decomposition(String::from("merchant")), 1);
        assert_eq!(
            Solution::longest_decomposition(String::from("antaprezatepzapreanta")),
            11
        );
    }
}
