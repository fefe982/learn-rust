// https://leetcode.com/problems/largest-number-after-mutating-substring/
// 1946. Largest Number After Mutating Substring
pub struct Solution;
impl Solution {
    pub fn maximum_number(num: String, change: Vec<i32>) -> String {
        let mut num = num.into_bytes();
        let mut changed = false;
        for b in &mut num {
            let d = (*b - b'0') as usize;
            if change[d] as u8 > *b - b'0' {
                *b = change[d] as u8 + b'0';
                changed = true;
            } else if changed && (change[d] as u8) < (*b - b'0') {
                break;
            }
        }
        String::from_utf8(num).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_number() {
        assert_eq!(
            Solution::maximum_number("132".to_string(), vec![9, 8, 5, 0, 3, 6, 4, 2, 6, 8]),
            "832".to_string()
        );
        assert_eq!(
            Solution::maximum_number("021".to_string(), vec![9, 4, 3, 5, 7, 2, 1, 9, 0, 6]),
            "934".to_string()
        );
        assert_eq!(
            Solution::maximum_number("5".to_string(), vec![1, 4, 7, 5, 3, 2, 5, 6, 9, 4]),
            "5".to_string()
        );
    }
}
