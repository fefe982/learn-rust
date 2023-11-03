// https://leetcode.com/problems/build-an-array-with-stack-operations/
// 1441. Build an Array With Stack Operations
pub struct Solution;
impl Solution {
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let mut last = 0;
        let mut res = vec![];
        for t in target {
            res.push("Push".to_string());
            for _ in 1..t - last {
                res.push("Pop".to_string());
                res.push("Push".to_string());
            }
            last = t;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;
    #[test]
    fn test_build_array() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec_str!["Push", "Push", "Pop", "Push"]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2, 3], 3),
            vec_str!["Push", "Push", "Push"]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2], 4),
            vec_str!["Push", "Push"]
        );
    }
}
