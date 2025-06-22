// https://leetcode.com/problems/queue-reconstruction-by-height/
// 406. Queue Reconstruction by Height
pub struct Solution;
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        people.sort_by(|a, b| if a[0] == b[0] { a[1].cmp(&b[1]) } else { b[0].cmp(&a[0]) });
        let mut res = vec![];
        for p in people {
            res.insert(p[1] as usize, p);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_reconstruct_queue() {
        assert_eq!(
            Solution::reconstruct_queue(vec_vec![[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]),
            vec![[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]]
        );
        assert_eq!(
            Solution::reconstruct_queue(vec_vec![[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]]),
            vec_vec![[4, 0], [5, 0], [2, 2], [3, 2], [1, 4], [6, 0]]
        );
    }
}
