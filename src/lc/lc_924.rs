// https://leetcode.com/problems/minimize-malware-spread/
// 924. Minimize Malware Spread
pub struct Solution;
impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let mut graph = graph;
        for i in 0..graph.len() {
            for j in 0..graph[i].len() {
                if graph[i][j] == 1 {
                    graph[j][i] = 1;
                }
            }
        }
        let mut initial = initial;
        initial.sort_unstable();
        let mut infect = vec![0; graph.len()];
        for &i in &initial {
            infect[i as usize] = 1;
        }
        let mut r = 0;
        let mut rc = -1;
        for i in initial {
            if infect[i as usize] == 2 {
                continue;
            }
            infect[i as usize] = 2;
            let mut ic = 1;
            let mut cnt = 1;
            let mut q = vec![i as usize];
            while let Some(qi) = q.pop() {
                for j in 0..graph.len() {
                    if infect[j] == 2 {
                        continue;
                    }
                    if graph[qi][j] == 1 {
                        cnt += 1;
                        q.push(j);
                        if infect[j] == 1 {
                            ic += 1;
                        }
                        infect[j] = 2;
                    }
                }
            }
            if ic == 1 {
                if cnt > rc {
                    rc = cnt;
                    r = i;
                }
            } else {
                if 0 > rc {
                    rc = 0;
                    r = i;
                }
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_malware_spread() {
        assert_eq!(
            Solution::min_malware_spread(vec_vec![[1, 1, 0], [1, 1, 0], [0, 0, 1]], vec![0, 1]),
            0
        );
        assert_eq!(
            Solution::min_malware_spread(vec_vec![[1, 0, 0], [0, 1, 0], [0, 0, 1]], vec![0, 2]),
            0
        );
        assert_eq!(
            Solution::min_malware_spread(vec_vec![[1, 1, 1], [1, 1, 1], [1, 1, 1]], vec![1, 2]),
            1
        );
    }
}
