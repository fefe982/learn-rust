// https://leetcode.com/problems/jump-game-iv/
// 1345. Jump Game IV
use std::collections::HashMap;
use std::collections::VecDeque;
pub struct Solution;
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return 0;
        }
        let mut same_number_map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (idx, n) in arr.iter().enumerate() {
            if let Some(idx_v) = same_number_map.get_mut(n) {
                idx_v.push(idx);
            } else {
                same_number_map.insert(*n, vec![idx]);
            }
        }
        let mut que: VecDeque<(usize, i32)> = VecDeque::new();
        que.push_back((0, 0));
        let mut visited = vec![false; arr.len()];
        visited[0] = true;
        while let Some((idx, step)) = que.pop_front() {
            if !visited[idx + 1] {
                if idx + 2 == arr.len() {
                    return step + 1;
                }
                que.push_back((idx + 1, step + 1));
                visited[idx + 1] = true;
            }
            if idx > 0 && !visited[idx - 1] {
                que.push_back((idx - 1, step + 1));
                visited[idx - 1] = true;
            }
            if let Some(v) = same_number_map.get_mut(&arr[idx]) {
                for idx_s in v.iter() {
                    if !visited[*idx_s] {
                        if idx_s + 1 == arr.len() {
                            return step + 1;
                        }
                        que.push_back((*idx_s, step + 1));
                        visited[*idx_s] = true;
                    }
                }
                v.clear();
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_jumps() {
        assert_eq!(
            Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
            3
        );
        assert_eq!(Solution::min_jumps(vec![7]), 0);
        assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
    }
}
