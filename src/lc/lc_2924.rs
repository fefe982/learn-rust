// https://leetcode.cn/problems/find-champion-ii/
// 2924. Find Champion II
pub struct Solution;
impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut v = vec![0; n as usize];
        for e in edges {
            v[e[1] as usize] += 1;
        }
        let mut root = usize::MAX;
        let mut cnt = 0;
        for (i, c) in v.into_iter().enumerate() {
            if c == 0 {
                cnt += 1;
                root = i;
            }
        }
        if cnt == 1 {
            root as i32
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_champion() {
        assert_eq!(Solution::find_champion(3, vec_vec![[0, 1], [1, 2]]), 0);
        assert_eq!(Solution::find_champion(4, vec_vec![[0, 2], [1, 3], [1, 2]]), -1);
    }
}
