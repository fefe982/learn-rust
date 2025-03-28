// https://leetcode.cn/problems/minimize-string-length/
// 2716. Minimize String Length
pub struct Solution;
impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let mut c = [false; 26];
        let mut l = 0;
        for b in s.bytes() {
            let idx = (b - b'a') as usize;
            if !c[idx] {
                c[idx] = true;
                l += 1;
            }
        }
        l
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimized_string_length() {
        assert_eq!(Solution::minimized_string_length("aaabc".to_string()), 3);
        assert_eq!(Solution::minimized_string_length("cbbd".to_string()), 3);
        assert_eq!(Solution::minimized_string_length("baadccab".to_string()), 4);
    }
}
