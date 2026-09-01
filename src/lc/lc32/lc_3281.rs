// https://leetcode.com/problems/maximize-score-of-numbers-in-ranges/
// 3281. Maximize Score of Numbers in Ranges
pub struct Solution;
impl Solution {
    pub fn max_possible_score(start: Vec<i32>, d: i64) -> i32 {
        let mut start = start;
        start.sort_unstable();
        let d = d as i64;
        let check = |v: i64| -> bool {
            let mut p = start[0] as i64;
            for i in 1..start.len() {
                let n = (start[i] as i64).max(p + v);
                if n > start[i] as i64 + d {
                    return false;
                }
                p = n;
            }
            true
        };
        let mut low = 0;
        let mut high = (*start.last().unwrap() as i64 - start[0] as i64 + d) / (start.len() as i64 - 1);
        if check(high) {
            return high as i32;
        }
        while low + 1 < high {
            let mid = (low + high) / 2;
            if check(mid) {
                low = mid;
            } else {
                high = mid;
            }
        }
        low as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_possible_score() {
        assert_eq!(Solution::max_possible_score(vec![6, 0, 3], 2), 4);
        assert_eq!(Solution::max_possible_score(vec![2, 6, 13, 13], 5), 5);
    }
}
