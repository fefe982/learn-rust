// https://leetcode.cn/problems/node-with-highest-edge-score/
// 2374. Node With the Highest Edge Score
pub struct Solution;
impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut ret = 0;
        let mut score = vec![0; edges.len()];
        for (i, &e) in edges.iter().enumerate() {
            let eu = e as usize;
            score[eu] += i as i64;
            if score[eu] > max {
                max = score[eu];
                ret = e;
            } else if score[eu] == max && e < ret {
                ret = e;
            }
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn edge_score() {
        assert_eq!(Solution::edge_score(vec![3, 3, 3, 0]), 0);
        assert_eq!(Solution::edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]), 7);
        assert_eq!(Solution::edge_score(vec![2, 0, 0, 2]), 0);
    }
}
