// https://leetcode.com/problems/shift-distance-between-two-strings/
// 3361. Shift Distance Between Two Strings
pub struct Solution;
impl Solution {
    pub fn shift_distance(s: String, t: String, next_cost: Vec<i32>, previous_cost: Vec<i32>) -> i64 {
        let mut next = vec![0; 27];
        let mut previous = vec![0; 27];
        for i in 0..26 {
            next[i + 1] = next[i] + next_cost[i] as i64;
        }
        for i in (0..26).rev() {
            previous[i] = previous[i + 1] + previous_cost[i] as i64;
        }
        let mut ans = 0;
        for (a, b) in s.chars().zip(t.chars()) {
            let ia = (a as u8 - b'a') as usize;
            let ib = (b as u8 - b'a') as usize;
            if ia > ib {
                let n = next[26] - next[ia] + next[ib];
                let p = previous[ib + 1] - previous[ia + 1];
                ans += n.min(p);
            } else {
                let n = next[ib] - next[ia];
                let p = previous[0] - previous[ia + 1] + previous[ib + 1];
                ans += n.min(p)
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shift_distance() {
        assert_eq!(
            Solution::shift_distance(
                "abab".to_string(),
                "baba".to_string(),
                vec![100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            2
        );
        assert_eq!(
            Solution::shift_distance(
                "leet".to_string(),
                "code".to_string(),
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
            ),
            31
        );
    }
}
