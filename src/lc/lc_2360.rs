// https://leetcode.com/problems/longest-cycle-in-a-graph/
// 2360. Longest Cycle in a Graph
pub struct Solution;
impl Solution {
    pub fn longest_cycle(mut edges: Vec<i32>) -> i32 {
        let mut max_cycle = -1;
        for idx in 0..edges.len() {
            if edges[idx] < 0 {
                continue;
            }
            let mut slow = edges[idx];
            if edges[slow as usize] < 0 {
                edges[idx] = -1;
                continue;
            }
            let mut fast = edges[slow as usize];
            while slow != fast {
                if edges[fast as usize] < 0 || edges[edges[fast as usize] as usize] < 0 {
                    let mut i = idx;
                    let mut ni = edges[i];
                    while ni >= 0 {
                        edges[i] = -1;
                        i = ni as usize;
                        ni = edges[i];
                    }
                    break;
                }
                fast = edges[edges[fast as usize] as usize];
                slow = edges[slow as usize];
            }
            if slow != fast {
                continue;
            }
            let mut cnt = 0;
            while edges[slow as usize] >= 0 {
                let nslow = edges[slow as usize];
                edges[slow as usize] = -1;
                slow = nslow;
                cnt += 1;
            }
            if cnt >= max_cycle {
                max_cycle = cnt;
            }
            let mut i = idx;
            let mut ni = edges[i];
            while ni >= 0 {
                edges[i] = -1;
                i = ni as usize;
                ni = edges[i];
            }
        }
        max_cycle
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_cycle() {
        assert_eq!(Solution::longest_cycle(vec![3, 3, 4, 2, 3]), 3);
        assert_eq!(Solution::longest_cycle(vec![2, -1, 3, 1]), -1);
    }
}
