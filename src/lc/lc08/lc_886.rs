// https://leetcode.com/problems/possible-bipartition/
// 886. Possible Bipartition
pub struct Solution;
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![vec![]; n as usize];
        for d in dislikes {
            graph[d[0] as usize - 1].push(d[1] as usize - 1);
            graph[d[1] as usize - 1].push(d[0] as usize - 1);
        }
        let mut color = vec![0; n as usize];
        for i in 0..n as usize {
            if color[i] == 0 {
                color[i] = 1;
                let mut q = vec![i];
                while let Some(c) = q.pop() {
                    for &n in &graph[c] {
                        if color[n] == 0 {
                            color[n] = -color[c];
                            q.push(n);
                        } else if color[n] == color[c] {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn possible_bipartition() {
        assert_eq!(
            Solution::possible_bipartition(4, vec_vec![[1, 2], [1, 3], [2, 4]]),
            true
        );
        assert_eq!(
            Solution::possible_bipartition(3, vec_vec![[1, 2], [1, 3], [2, 3]]),
            false
        );
    }
}
