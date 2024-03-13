// https://leetcode.com/problems/max-value-of-equation/
// 1499. Max Value of Equation
pub struct Solution;
impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut m = i32::MIN;
        let mut hp = std::collections::BinaryHeap::new();
        for point in &points {
            while let Some(&(s, x)) = hp.peek() {
                if point[0] - x > k {
                    hp.pop();
                } else {
                    m = m.max(s + point[0] + point[1]);
                    break;
                }
            }
            hp.push((point[1] - point[0], point[0]));
        }
        m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_max_value_of_equation() {
        assert_eq!(
            Solution::find_max_value_of_equation(
                vec_vec![
                    [-13, 19],
                    [-6, -16],
                    [2, -7],
                    [6, -13],
                    [8, -16],
                    [13, 6],
                    [14, -13],
                    [20, 4]
                ],
                3
            ),
            -6
        );
        assert_eq!(
            Solution::find_max_value_of_equation(
                vec_vec![
                    [-15, -1],
                    [-14, -5],
                    [-11, 1],
                    [-9, 7],
                    [-8, 18],
                    [-7, -5],
                    [-3, 3],
                    [4, 14],
                    [12, -4],
                    [13, 15],
                    [14, -19],
                    [19, -1]
                ],
                8
            ),
            26
        );
        assert_eq!(
            Solution::find_max_value_of_equation(vec_vec![[1, 3], [2, 0], [5, 10], [6, -10]], 1),
            4
        );
        assert_eq!(
            Solution::find_max_value_of_equation(vec_vec![[0, 0], [3, 0], [9, 2]], 3),
            3
        );
    }
}
