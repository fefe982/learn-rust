// https://leetcode.com/problems/count-zero-request-servers/
// 2747. Count Zero Request Servers
pub struct Solution;
impl Solution {
    pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; queries.len()];
        let mut logs = logs;
        let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
        logs.sort_unstable_by_key(|v| v[1]);
        queries.sort_unstable_by_key(|&(_, x)| x);
        let mut i = 0;
        let mut j = 0;
        let mut seen = std::collections::HashMap::new();
        for (iq, q) in queries {
            while i < logs.len() && logs[i][1] < q - x {
                if let Some(&ilast) = seen.get(&logs[i][0]) {
                    if ilast < q - x {
                        seen.remove(&logs[i][0]);
                    }
                }
                i += 1;
            }
            if j < i {
                j = i;
            }
            while j < logs.len() && logs[j][1] <= q {
                seen.insert(logs[j][0], logs[j][1]);
                j += 1;
            }
            ans[iq] = n - seen.len() as i32;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_servers() {
        assert_eq!(
            Solution::count_servers(3, vec_vec![[1, 3], [2, 6], [1, 5]], 5, vec![10, 11]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::count_servers(3, vec_vec![[2, 4], [2, 1], [1, 2], [3, 1]], 2, vec![3, 4]),
            vec![0, 1]
        );
    }
}
