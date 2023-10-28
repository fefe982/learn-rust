// https://leetcode.com/problems/move-pieces-to-obtain-a-string/
// 2337. Move Pieces to Obtain a String
pub struct Solution;
impl Solution {
    fn cvt(s: String) -> Vec<(u8, usize)> {
        s.as_bytes()
            .into_iter()
            .enumerate()
            .filter_map(|(i, &c)| if c != b'_' { Some((c, i)) } else { None })
            .collect()
    }
    pub fn can_change(start: String, target: String) -> bool {
        let s = Self::cvt(start);
        let t = Self::cvt(target);
        if s.len() != t.len() {
            return false;
        }
        for ((cs, is), (ct, it)) in s.into_iter().zip(t.into_iter()) {
            if cs != ct {
                return false;
            }
            if cs == b'L' && it > is {
                return false;
            }
            if cs == b'R' && it < is {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_change() {
        assert_eq!(
            Solution::can_change("_L__R__R_".to_string(), "L______RR".to_string()),
            true
        );
        assert_eq!(
            Solution::can_change("R_L_".to_string(), "__LR".to_string()),
            false
        );
        assert_eq!(
            Solution::can_change("_R".to_string(), "R_".to_string()),
            false
        );
    }
}
