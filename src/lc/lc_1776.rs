// https://leetcode.com/problems/car-fleet-ii/
// 1776. Car Fleet II
pub struct Solution;
impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        let mut stack = vec![];
        let mut ans = vec![0.0; cars.len()];
        for (i, car) in cars.iter().enumerate().rev() {
            while let Some(&(start, speed, end)) = stack.last() {
                if car[1] <= speed {
                    stack.pop();
                    continue;
                }
                let collide = (start - car[0]) as f64 / (car[1] - speed) as f64;
                if collide <= end {
                    stack.push((car[0], car[1], collide));
                    ans[i] = collide;
                    break;
                } else {
                    stack.pop();
                }
            }
            if stack.is_empty() {
                stack.push((car[0], car[1], f64::MAX));
                ans[i] = -1.0;
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
    fn test_get_collision_times() {
        assert_eq!(
            Solution::get_collision_times(vec_vec![[1, 2], [2, 1], [4, 3], [7, 2]]),
            vec![1.00000, -1.00000, 3.00000, -1.00000]
        );
        assert_eq!(
            Solution::get_collision_times(vec_vec![[3, 4], [5, 4], [6, 3], [9, 1]]),
            vec![2.00000, 1.00000, 1.50000, -1.00000]
        );
    }
}
