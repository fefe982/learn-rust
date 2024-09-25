// https://leetcode.com/problems/count-visited-nodes-in-a-directed-graph/
// 2876. Count Visited Nodes in a Directed Graph
pub struct Solution;
impl Solution {
    pub fn count_visited_nodes(edges: Vec<i32>) -> Vec<i32> {
        let n = edges.len();
        let mut visited = vec![0; n];
        let mut res = vec![0; n];
        for i in 0..n {
            if res[i] > 0 {
                continue;
            }
            let mut cnt = 1;
            visited[i] = 1;
            let mut end = edges[i] as usize;
            while res[end] == 0 && visited[end] == 0 {
                cnt += 1;
                visited[end] = cnt;
                end = edges[end] as usize;
            }
            if res[end] == 0 {
                let lp = cnt - visited[end] + 1;
                cnt -= lp;
                res[end] = lp;
                let mut j = edges[end] as usize;
                while j != end {
                    res[j] = lp;
                    j = edges[j] as usize;
                }
            }
            let mut j = i;
            while j != end {
                res[j] = cnt + res[end];
                cnt -= 1;
                j = edges[j] as usize;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_visited_nodes() {
        assert_eq!(Solution::count_visited_nodes(vec![1, 2, 0, 0]), [3, 3, 3, 4]);
        assert_eq!(Solution::count_visited_nodes(vec![1, 2, 3, 4, 0]), [5, 5, 5, 5, 5]);
    }
}
