// https://leetcode.com/problems/sliding-puzzle/
// 773. Sliding Puzzle
pub struct Solution;
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let end = vec![vec![1, 2, 3], vec![4, 5, 0]];
        if board == end {
            return 0;
        }
        let mut visited = std::collections::HashSet::new();
        let mut q = std::collections::VecDeque::new();
        q.push_back((board, 0));
        while let Some((cur, step)) = q.pop_front() {
            if visited.contains(&cur) {
                continue;
            }
            visited.insert(cur.clone());
            let mut pi = 0;
            let mut pj = 0;
            for i in 0..2 {
                for j in 0..3 {
                    if cur[i][j] == 0 {
                        pi = i;
                        pj = j;
                        break;
                    }
                }
            }
            let mut next = cur.clone();
            next[pi][pj] = cur[1 - pi][pj];
            next[1 - pi][pj] = 0;
            if next == end {
                return step + 1;
            }
            if !visited.contains(&next) {
                q.push_back((next, step + 1));
            }
            if pj > 0 {
                next = cur.clone();
                next[pi][pj] = cur[pi][pj - 1];
                next[pi][pj - 1] = 0;
                if next == end {
                    return step + 1;
                }
                if !visited.contains(&next) {
                    q.push_back((next, step + 1));
                }
            }
            if pj < 2 {
                next = cur.clone();
                next[pi][pj] = cur[pi][pj + 1];
                next[pi][pj + 1] = 0;
                if next == end {
                    return step + 1;
                }
                if !visited.contains(&next) {
                    q.push_back((next, step + 1));
                }
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
    fn test_sliding_puzzle() {
        assert_eq!(Solution::sliding_puzzle(vec_vec![[1, 2, 3], [4, 0, 5]]), 1);
        assert_eq!(Solution::sliding_puzzle(vec_vec![[1, 2, 3], [5, 4, 0]]), -1);
        assert_eq!(Solution::sliding_puzzle(vec_vec![[4, 1, 2], [5, 0, 3]]), 5);
    }
}
