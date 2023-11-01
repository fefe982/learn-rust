// https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/
// 2127. Maximum Employees to Be Invited to a Meeting
pub struct Solution;
impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let mut p = vec![std::collections::HashSet::<i32>::new(); favorite.len()];
        for (i, &f) in favorite.iter().enumerate() {
            p[f as usize].insert(i as i32);
        }
        let mut l = vec![1; favorite.len()];
        let mut q = vec![];
        for (i, pv) in p.iter().enumerate() {
            if pv.is_empty() {
                q.push(i);
                l[i] = 1;
            }
        }
        let mut visited = vec![false; favorite.len()];
        while let Some(i) = q.pop() {
            visited[i] = true;
            let j = favorite[i] as usize;
            p[j].remove(&(i as i32));
            l[j] = l[j].max(l[i] + 1);
            if p[j].is_empty() {
                q.push(j);
            }
        }
        let mut max_ring = 0;
        let mut max_2 = 0;
        for i in 0..favorite.len() {
            if !visited[i] {
                visited[i] = true;
                let mut ring = 1;
                let mut j = favorite[i] as usize;
                while !visited[j] {
                    visited[j] = true;
                    ring += 1;
                    j = favorite[j] as usize;
                }
                if ring == 2 {
                    max_2 += l[i] + l[favorite[i] as usize];
                } else {
                    max_ring = max_ring.max(ring);
                }
            }
        }
        max_2.max(max_ring)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_invitations() {
        assert_eq!(
            Solution::maximum_invitations(vec![1, 0, 3, 2, 5, 6, 7, 4, 9, 8, 11, 10, 11, 12, 10]),
            11
        );
        assert_eq!(Solution::maximum_invitations(vec![2, 2, 1, 2]), 3);
        assert_eq!(Solution::maximum_invitations(vec![1, 2, 0]), 3);
        assert_eq!(Solution::maximum_invitations(vec![3, 0, 1, 4, 1]), 4);
    }
}
