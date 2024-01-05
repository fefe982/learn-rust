// https://leetcode.com/problems/number-of-visible-people-in-a-queue/
// 1944. Number of Visible People in a Queue
pub struct Solution;
impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut r = vec![];
        let mut a = vec![];
        for h in heights.into_iter().rev() {
            let pos = r.partition_point(|&x| x >= h);
            a.push((r.len() - pos) as i32 + if pos != 0 { 1 } else { 0 });
            while let Some(&i) = r.last() {
                if i < h {
                    r.pop();
                } else {
                    break;
                }
            }
            r.push(h);
        }
        a.reverse();
        a
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_see_persons_count() {
        assert_eq!(
            Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9]),
            [3, 1, 2, 1, 1, 0]
        );
        assert_eq!(Solution::can_see_persons_count(vec![5, 1, 2, 3, 10]), [4, 1, 1, 1, 0]);
    }
}
