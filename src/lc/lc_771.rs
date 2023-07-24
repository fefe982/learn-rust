// https://leetcode.com/problems/jewels-and-stones/
// 771. Jewels and Stones
pub struct Solution;
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut t = std::collections::HashSet::new();
        jewels.chars().for_each(|c| {
            t.insert(c);
        });
        let mut cnt = 0;
        stones.chars().for_each(|c| {
            if t.contains(&c) {
                cnt += 1;
            }
        });
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_jewels_in_stones() {
        assert_eq!(
            Solution::num_jewels_in_stones(String::from("aA"), String::from("aAAbbbb")),
            3
        );
        assert_eq!(
            Solution::num_jewels_in_stones(String::from("z"), String::from("ZZ")),
            0
        )
    }
}
