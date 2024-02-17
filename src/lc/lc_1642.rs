// https://leetcode.com/problems/furthest-building-you-can-reach/
// 1642. Furthest Building You Can Reach
pub struct Solution;
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut q = std::collections::BinaryHeap::new();
        let mut last = heights[0];
        let mut sum = 0;
        for i in 1..heights.len() {
            if heights[i] > last {
                q.push(std::cmp::Reverse(heights[i] - last));
            }
            if q.len() > ladders as usize {
                sum += q.pop().unwrap().0;
                if sum > bricks {
                    return i as i32 - 1;
                }
            }
            last = heights[i];
        }
        heights.len() as i32 - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_furthest_building() {
        assert_eq!(Solution::furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1), 4);
        assert_eq!(
            Solution::furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2),
            7
        );
        assert_eq!(Solution::furthest_building(vec![14, 3, 19, 3], 17, 0), 3);
    }
}
