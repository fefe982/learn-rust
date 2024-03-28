// https://leetcode.com/problems/find-servers-that-handled-most-number-of-requests/
// 1606. Find Servers That Handled Most Number of Requests
pub struct Solution;
impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let mut after = std::collections::BinaryHeap::new();
        let mut before = std::collections::BinaryHeap::from_iter((0..k).map(|x| std::cmp::Reverse(x as usize)));
        let mut job = std::collections::BinaryHeap::<std::cmp::Reverse<(i32, usize)>>::new();
        let mut cnt = vec![0; k as usize];
        let mut max_cnt = 0;
        for (idx, (arrive, load)) in arrival.into_iter().zip(load.into_iter()).enumerate() {
            let node_idx = idx % k as usize;
            if node_idx == 0 {
                after = before;
                before = std::collections::BinaryHeap::new();
            }
            while let Some(&std::cmp::Reverse((end, node))) = job.peek() {
                if end <= arrive {
                    job.pop();
                    if node < node_idx {
                        before.push(std::cmp::Reverse(node));
                    } else {
                        after.push(std::cmp::Reverse(node));
                    }
                } else {
                    break;
                }
            }
            if let Some(node) = if let Some(std::cmp::Reverse(node)) = after.pop() {
                Some(node)
            } else if let Some(std::cmp::Reverse(node)) = before.pop() {
                Some(node)
            } else {
                None
            } {
                job.push(std::cmp::Reverse((arrive + load, node)));
                cnt[node] += 1;
                max_cnt = max_cnt.max(cnt[node]);
            }
        }
        cnt.into_iter()
            .enumerate()
            .filter_map(|(i, x)| if x == max_cnt { Some(i as i32) } else { None })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_busiest_servers() {
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3]),
            vec![1]
        );
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3, 4], vec![1, 2, 1, 2]),
            vec![0]
        );
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3], vec![10, 12, 11]),
            vec![0, 1, 2]
        );
    }
}
