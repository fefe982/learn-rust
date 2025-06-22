// https://leetcode.com/problems/swap-adjacent-in-lr-string/
// 777. Swap Adjacent in LR String
pub struct Solution;
impl Solution {
    pub fn can_transform(start: String, result: String) -> bool {
        let start = start.as_bytes();
        let result = result.as_bytes();
        let mut i = 0;
        let mut j = 0;
        loop {
            while i < start.len() && start[i] == b'X' {
                i += 1;
            }
            while j < result.len() && result[j] == b'X' {
                j += 1;
            }
            if i >= start.len() && j >= result.len() {
                return true;
            }
            if i >= start.len() || j >= result.len() {
                return false;
            }
            if start[i] == result[j] {
                if start[i] == b'L' && i < j {
                    println!("L {i} {j}");
                    return false;
                }
                if start[i] == b'R' && i > j {
                    return false;
                }
                i += 1;
                j += 1;
            } else {
                return false;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_transform() {
        assert_eq!(
            Solution::can_transform("RXXLRXRXL".to_string(), "XRLXXRRLX".to_string()),
            true
        );
        assert_eq!(Solution::can_transform("X".to_string(), "L".to_string()), false);
    }
}
