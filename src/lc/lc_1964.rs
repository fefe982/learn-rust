// https://leetcode.com/problems/find-the-longest-valid-obstacle-course-at-each-position/
// 1964. Find the Longest Valid Obstacle Course at Each Position
pub struct Solution;
impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        // min_ele[i] holds the minimum ending element forming a non decreasing sub sequence
        // of length i + 1
        let mut min_ele = Vec::new();
        let mut res = Vec::with_capacity(obstacles.capacity());
        for o in obstacles {
            let idx = min_ele.partition_point(|&x| x <= o);
            if idx == min_ele.len() {
                min_ele.push(o);
            } else {
                min_ele[idx] = o;
            }
            res.push(idx as i32 + 1);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_obstacle_course_at_each_position() {
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(vec![1, 2, 3, 2]),
            vec![1, 2, 3, 3]
        );
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(vec![2, 2, 1]),
            vec![1, 2, 1]
        );
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(vec![3, 1, 5, 6, 4, 2]),
            vec![1, 1, 2, 3, 2, 2]
        )
    }
}
