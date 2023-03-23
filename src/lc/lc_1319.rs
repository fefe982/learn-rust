// https://leetcode.com/problems/number-of-operations-to-make-network-connected/
// 1319. Number of Operations to Make Network Connected
pub struct Solution;
impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if connections.len() + 1 < n {
            return -1;
        }
        if connections.len() > (n - 1) * (n - 2) / 2 {
            return 0;
        }
        let mut conn = vec![vec![]; n];
        for connection in connections {
            conn[connection[0] as usize].push(connection[1] as usize);
            conn[connection[1] as usize].push(connection[0] as usize);
        }
        let mut visited = vec![false; n];
        let mut blocks = 0;
        for idx in 0..n {
            if visited[idx] {
                continue;
            }
            blocks += 1;
            let mut check = vec![idx];
            while let Some(idx_check) = check.pop() {
                if visited[idx_check] {
                    continue;
                }
                visited[idx_check] = true;
                for dst in &conn[idx_check] {
                    check.push(*dst);
                }
            }
        }
        blocks - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_connected() {
        assert_eq!(
            Solution::make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
            1
        );
        assert_eq!(
            Solution::make_connected(
                6,
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]
            ),
            2
        );
        assert_eq!(
            Solution::make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]),
            -1
        );
    }
}
