// https://leetcode.com/problems/number-of-ways-to-select-buildings/
// 2222. Number of Ways to Select Buildings
pub struct Solution;
impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        let mut count = [[0; 3]; 2];
        for c in s.bytes() {
            let idx = (c - b'0') as usize;
            count[idx][2] += count[1 - idx][1];
            count[idx][1] += count[1 - idx][0];
            count[idx][0] += 1;
        }
        count[0][2] + count[1][2]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_ways() {
        assert_eq!(Solution::number_of_ways("001101".to_string()), 6);
        assert_eq!(Solution::number_of_ways("11100".to_string()), 0);
    }
}
