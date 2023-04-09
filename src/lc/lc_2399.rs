// https://leetcode.com/problems/check-distances-between-same-letters/description/
// 2399. Check Distances Between Same Letters
pub struct Solution;
impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut bs: Vec<u8> = s.bytes().collect();
        for idx in 0..bs.len() {
            if bs[idx] == 0 {
                continue;
            }
            let next = idx + distance[(bs[idx] - b'a') as usize] as usize + 1;
            if next < bs.len() && bs[next] == bs[idx] {
                bs[next] = 0;
            } else {
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
    fn check_distances() {
        assert_eq!(
            Solution::check_distances(
                String::from("abaccb"),
                vec![1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            true
        );
        assert_eq!(
            Solution::check_distances(
                String::from("aa"),
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            false
        );
    }
}
