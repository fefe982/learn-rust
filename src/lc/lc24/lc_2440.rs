// https://leetcode.com/problems/create-components-with-same-value/
// 2440. Create Components With Same Value
pub struct Solution;
impl Solution {
    fn check(
        mut cluster: Vec<i32>,
        mut graph: Vec<std::collections::HashSet<usize>>,
        mut q: std::collections::VecDeque<usize>,
        val: i32,
    ) -> bool {
        while let Some(n) = q.pop_front() {
            if graph[n].len() == 0 {
                if cluster[n] != val {
                    return false;
                }
            } else {
                let &next = graph[n].iter().next().unwrap();
                graph[next].remove(&n);
                if cluster[n] < val {
                    cluster[next] += cluster[n];
                    if cluster[next] > val {
                        return false;
                    }
                }
                if graph[next].len() == 1 {
                    q.push_back(next);
                }
            }
        }
        true
    }
    pub fn component_value(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut graph = vec![std::collections::HashSet::new(); n];
        for edge in edges {
            graph[edge[0] as usize].insert(edge[1] as usize);
            graph[edge[1] as usize].insert(edge[0] as usize);
        }
        let mut max = 0;
        let mut sum = 0;
        for &num in nums.iter() {
            max = max.max(num);
            sum += num;
        }
        let mut q = std::collections::VecDeque::new();
        for (i, s) in graph.iter().enumerate() {
            if s.len() <= 1 {
                q.push_back(i);
            }
        }
        let mut i = 1;
        let mut max_split = 0;
        while i * i <= sum {
            if sum % i == 0 {
                let j = sum / i;
                if i >= max && Self::check(nums.clone(), graph.clone(), q.clone(), i) {
                    max_split = max_split.max(j - 1);
                }
                if j != i && j >= max && Self::check(nums.clone(), graph.clone(), q.clone(), j) {
                    max_split = max_split.max(i - 1);
                }
            }
            i += 1;
        }
        max_split
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_component_value() {
        assert_eq!(
            Solution::component_value(vec![6, 2, 2, 2, 6], vec_vec![[0, 1], [1, 2], [1, 3], [3, 4]]),
            2
        );
        assert_eq!(Solution::component_value(vec![2], vec_vec![]), 0);
    }
}
