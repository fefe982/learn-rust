// https://leetcode.com/problems/partition-labels/
// 763. Partition Labels
pub struct Solution;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut pos = [[usize::MAX; 2]; 27];
        for (i, &c) in s.as_bytes().iter().enumerate() {
            let idx = (c - b'a') as usize;
            pos[idx][0] = pos[idx][0].min(i);
            pos[idx][1] = i;
        }
        pos.sort_unstable();
        let mut res = vec![];
        let mut beg = pos[0][0];
        let mut end = pos[0][1];
        for i in 1..27 {
            if pos[i][0] > end {
                res.push((end - beg + 1) as i32);
                beg = pos[i][0];
                end = pos[i][1];
            } else {
                end = end.max(pos[i][1]);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partition_labels() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
            [9, 7, 8]
        );
        assert_eq!(Solution::partition_labels("eccbbbbdec".to_string()), [10]);
    }
}
