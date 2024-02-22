// https://leetcode.com/problems/find-the-town-judge/
// 997. Find the Town Judge
pub struct Solution;
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut trusts = vec![0; n as usize + 1];
        let mut trusted = vec![0; n as usize + 1];
        for t in trust {
            trusts[t[0] as usize] += 1;
            trusted[t[1] as usize] += 1;
        }
        for i in 1..=n as usize {
            if trusts[i] == 0 && trusted[i] == n as usize - 1 {
                return i as i32;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_judge() {
        assert_eq!(Solution::find_judge(2, vec_vec![[1, 2]]), 2);
        assert_eq!(Solution::find_judge(3, vec_vec![[1, 3], [2, 3]]), 3);
        assert_eq!(Solution::find_judge(3, vec_vec![[1, 3], [2, 3], [3, 1]]), -1);
    }
}
