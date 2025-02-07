// https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/
// 3160. Find the Number of Distinct Colors Among the Balls
pub struct Solution;
impl Solution {
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ball = std::collections::HashMap::<i32, i32>::new();
        let mut color = std::collections::HashMap::<i32, i32>::new();
        let mut res = Vec::with_capacity(queries.len());
        let mut cnt = 0;
        for q in queries {
            if let Some(c) = ball.insert(q[0], q[1]) {
                let ccnt = color.entry(c).or_default();
                *ccnt -= 1;
                if *ccnt == 0 {
                    cnt -= 1;
                }
            };
            let ccnt = color.entry(q[1]).and_modify(|c| *c += 1).or_insert(1);
            if *ccnt == 1 {
                cnt += 1;
            }
            res.push(cnt);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn query_results() {
        assert_eq!(
            Solution::query_results(4, vec_vec![[1, 4], [2, 5], [1, 3], [3, 4]]),
            [1, 2, 2, 3]
        );
        assert_eq!(
            Solution::query_results(4, vec_vec![[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]]),
            [1, 2, 2, 3, 4]
        );
    }
}
