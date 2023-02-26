// https://leetcode.com/problems/edit-distance/
// 72. Edit Distance
use std::{cmp::min, mem::swap};
pub struct Solution;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut min_dis = Vec::from_iter(0..=(word1.len() as i32));
        let mut new_min_dis: Vec<i32> = Vec::with_capacity(min_dis.len());
        new_min_dis.resize(min_dis.len(), 0);
        let ch1 = word1.as_bytes();
        let ch2 = word2.as_bytes();
        for idx2 in 1..=word2.len() {
            new_min_dis[0] = idx2 as i32;
            for idx1 in 1..=word1.len() {
                let rcost = if ch1[idx1 - 1] == ch2[idx2 - 1] { 0 } else { 1 };
                new_min_dis[idx1] = min(
                    min(min_dis[idx1 - 1] + rcost, min_dis[idx1] + 1),
                    new_min_dis[idx1 - 1] + 1,
                );
            }
            swap(&mut min_dis, &mut new_min_dis);
        }
        min_dis[word1.len()]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_distance() {
        assert_eq!(
            Solution::min_distance(String::from("horse"), String::from("ros")),
            3
        );
        assert_eq!(
            Solution::min_distance(String::from("intention"), String::from("execution")),
            5
        );
    }
}
