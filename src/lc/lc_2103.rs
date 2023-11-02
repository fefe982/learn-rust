// https://leetcode.com/problems/rings-and-rods/
// 2103. Rings and Rods
pub struct Solution;
impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let rings = rings.as_bytes();
        let mut cnt = vec![0; 10];
        for i in 0..rings.len() / 2 {
            let pos = i * 2;
            let idx = (rings[pos + 1] - b'0') as usize;
            match rings[pos] {
                b'R' => cnt[idx] |= 1,
                b'G' => cnt[idx] |= 2,
                b'B' => cnt[idx] |= 4,
                _ => unreachable!(),
            }
        }
        cnt.into_iter()
            .fold(0, |acc, x| acc + if x == 7 { 1 } else { 0 })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_points() {
        assert_eq!(Solution::count_points("B0B6G0R6R0R6G9".to_string()), 1);
        assert_eq!(Solution::count_points("B0R0G0R9R0B0G0".to_string()), 1);
        assert_eq!(Solution::count_points("G4".to_string()), 0);
    }
}
