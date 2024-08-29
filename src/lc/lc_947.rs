// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/
// 947. Most Stones Removed with Same Row or Column
pub struct Solution;
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut parent = vec![usize::MAX; 10001];
        let get_p = |parent: &mut Vec<usize>, x: usize| {
            let mut p = parent[x];
            while parent[p] != p {
                p = parent[p];
            }
            parent[x] = p;
            p
        };
        let mut cnt = 0;
        let mut stones = stones;
        stones.sort();
        for i in 0..stones.len() {
            if i == 0 || stones[i - 1][0] != stones[i][0] {
                if parent[stones[i][1] as usize] == usize::MAX {
                    cnt += 1;
                    parent[stones[i][1] as usize] = stones[i][1] as usize;
                }
            } else {
                let pp = get_p(&mut parent, stones[i - 1][1] as usize);
                if parent[stones[i][1] as usize] == usize::MAX {
                    parent[stones[i][1] as usize] = pp;
                } else {
                    let pi = get_p(&mut parent, stones[i][1] as usize);
                    if pi != pp {
                        parent[pp] = pi;
                        cnt -= 1;
                    }
                }
            }
        }
        stones.len() as i32 - cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_remove_stones() {
        assert_eq!(Solution::remove_stones(vec_vec![[0, 1], [1, 0], [1, 1]]), 2);
        assert_eq!(
            Solution::remove_stones(vec_vec![[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]]),
            5
        );
        assert_eq!(
            Solution::remove_stones(vec_vec![[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]]),
            3
        );
        assert_eq!(Solution::remove_stones(vec_vec![[0, 0]]), 0);
    }
}
