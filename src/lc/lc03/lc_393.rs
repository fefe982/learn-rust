// https://leetcode.com/problems/utf-8-validation/
// 393. UTF-8 Validation
pub struct Solution;
impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut cont = 0;
        for d in data {
            if cont > 0 {
                if d & 0xc0 == 0x80 {
                    cont -= 1;
                } else {
                    return false;
                }
            } else {
                if d & 0x80 == 0 {
                    continue;
                } else if d & 0xe0 == 0xc0 {
                    cont = 1;
                } else if d & 0xf0 == 0xe0 {
                    cont = 2;
                } else if d & 0xf8 == 0xf0 {
                    cont = 3;
                } else {
                    return false;
                }
            }
        }
        cont == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_utf8() {
        assert_eq!(Solution::valid_utf8(vec![197, 130, 1]), true);
        assert_eq!(Solution::valid_utf8(vec![235, 140, 4]), false);
    }
}
