// https://leetcode.com/problems/find-closest-node-to-given-two-nodes/
// 2359. Find Closest Node to Given Two Nodes
pub struct Solution;
impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let mut dist = vec![-1; edges.len()];
        let mut lp = vec![-1; edges.len()];
        let mut cur = node1 as usize;
        dist[cur] = 0;
        while edges[cur] != -1 && dist[edges[cur] as usize] == -1 {
            dist[edges[cur] as usize] = dist[cur] + 1;
            cur = edges[cur] as usize;
        }
        let mut lp_entry = -1;
        if edges[cur] != -1 {
            lp_entry = edges[cur];
            let lp_len = dist[cur] + 1 - dist[edges[cur] as usize];
            while lp[cur] == -1 {
                lp[cur] = lp_len;
                cur = edges[cur] as usize;
            }
        }
        cur = node2 as usize;
        let mut d = 0;
        loop {
            if dist[cur] >= 0 {
                if lp[cur] == -1 {
                    return cur as i32;
                } else {
                    let c = dist[cur].max(d);
                    let diff_entry = dist[cur] - dist[lp_entry as usize];
                    let d_entry = (d + lp[cur] - diff_entry).max(dist[lp_entry as usize]);
                    if d_entry < c {
                        return lp_entry;
                    } else if d_entry == c {
                        return lp_entry.min(cur as i32);
                    } else {
                        return cur as i32;
                    }
                }
            }
            d += 1;
            dist[cur] = -2;
            if edges[cur] == -1 {
                return -1;
            }
            cur = edges[cur] as usize;
            if dist[cur] == -2 {
                return -1;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn closest_meeting_node() {
        assert_eq!(Solution::closest_meeting_node(vec![5, 3, 1, 0, 2, 4, 5], 3, 2), 3);
        assert_eq!(Solution::closest_meeting_node(vec![4, 3, 0, 5, 3, -1], 4, 0), 4);
        assert_eq!(Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1), 2);
        assert_eq!(Solution::closest_meeting_node(vec![1, 2, -1], 0, 2), 2);
    }
}
