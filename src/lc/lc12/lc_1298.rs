// https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/
// 1298. Maximum Candies You Can Get from Boxes
pub struct Solution;
impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut status = status;
        let mut boxes = initial_boxes;
        let mut ans = 0;
        while let Some(b) = boxes.pop() {
            if status[b as usize] == 1 {
                for &key in &keys[b as usize] {
                    if status[key as usize] == 2 {
                        boxes.push(key);
                    }
                    status[key as usize] = 1;
                }
                ans += candies[b as usize];
                for &cb in &contained_boxes[b as usize] {
                    boxes.push(cb);
                }
            } else if status[b as usize] == 0 {
                status[b as usize] = 2;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_candies() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 1, 0],
                vec![7, 5, 4, 100],
                vec_vec![[], [], [1], []],
                vec_vec![[1, 2], [3], [], []],
                vec![0]
            ),
            16
        );
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1],
                vec_vec![[1, 2, 3, 4, 5], [], [], [], [], []],
                vec_vec![[1, 2, 3, 4, 5], [], [], [], [], []],
                vec![0]
            ),
            6
        );
    }
}
