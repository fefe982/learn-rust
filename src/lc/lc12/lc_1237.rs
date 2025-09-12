// https://leetcode.com/problems/find-positive-integer-solution-for-a-given-equation/
// 1237. Find Positive Integer Solution for a Given Equation
pub trait CustomFunction {
    fn f(&self, x: i32, y: i32) -> i32;
}
pub struct Solution;
impl Solution {
    pub fn find_solution(customfunction: &impl CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let (mut x, mut y) = (1, 1000);
        while x <= 1000 && y >= 1 {
            let val = customfunction.f(x, y);
            if val == z {
                res.push(vec![x, y]);
                x += 1;
                y -= 1;
            } else if val < z {
                x += 1;
            } else {
                y -= 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    struct Add;
    impl CustomFunction for Add {
        fn f(&self, x: i32, y: i32) -> i32 {
            x + y
        }
    }
    struct Mul;
    impl CustomFunction for Mul {
        fn f(&self, x: i32, y: i32) -> i32 {
            x * y
        }
    }
    #[test]
    fn find_solution() {
        assert_sort_eq!(
            Solution::find_solution(&Add {}, 5),
            vec_vec![[1, 4], [2, 3], [3, 2], [4, 1]]
        );
        assert_sort_eq!(Solution::find_solution(&Mul {}, 5), vec_vec![[1, 5], [5, 1]]);
    }
}
