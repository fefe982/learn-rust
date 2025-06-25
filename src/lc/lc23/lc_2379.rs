// https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/
// 2379. Minimum Recolors to Get K Consecutive Black Blocks
pub struct Solution;
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let bytes = blocks.as_bytes();
        let len = bytes.len();
        let k = k as usize;
        let mut current_cnt = 0;
        for i in 0..k {
            if bytes[i] == 'W' as u8 {
                current_cnt += 1;
            }
        }
        let mut min_cnt = current_cnt;
        for i in k..len {
            if bytes[i] == 'W' as u8 {
                current_cnt += 1;
            }
            if bytes[i - k] == 'W' as u8 {
                current_cnt -= 1;
            }
            if current_cnt < min_cnt {
                min_cnt = current_cnt;
            }
        }
        min_cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_recolors() {
        assert_eq!(Solution::minimum_recolors(String::from("WBBWWBBWBW"), 7), 3);
        assert_eq!(Solution::minimum_recolors(String::from("WBWBBBW"), 2), 0);
    }
}
