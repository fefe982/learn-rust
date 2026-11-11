// https://leetcode.com/problems/remove-k-balanced-substrings/
// 3703. Remove K Balanced Substrings
pub struct Solution;
impl Solution {
    pub fn remove_substring(s: String, k: i32) -> String {
        let mut stack = vec![];
        let mut l = 0;
        let mut r = 0;
        let mut last = '.';
        for c in s.chars() {
            if c == '(' {
                if last == '(' {
                    l += 1;
                } else {
                    l = 1;
                    r = 0;
                }
                stack.push((c, l, r));
                last = c;
            } else {
                r += 1;
                if l >= k && r >= k {
                    for _ in 1..k * 2 {
                        stack.pop();
                    }
                    if let Some(&(cc, ll, rr)) = stack.last() {
                        last = cc;
                        l = ll;
                        r = rr;
                    } else {
                        last = '.';
                        l = 0;
                        r = 0;
                    }
                } else {
                    stack.push((c, l, r));
                    last = c;
                }
            }
        }
        stack.into_iter().map(|(c, _, _)| c).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_substring() {
        assert_eq!(Solution::remove_substring("(())".to_string(), 1), "".to_string());
        assert_eq!(Solution::remove_substring("(()(".to_string(), 1), "((".to_string());
        assert_eq!(
            Solution::remove_substring("((()))()()()".to_string(), 3),
            "()()()".to_string()
        );
    }
}
