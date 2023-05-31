// https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/
// 1130. Minimum Cost Tree From Leaf Values
pub struct Solution;
impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut sum = 0;
        for n in arr {
            loop {
                if let Some(&x) = stack.last() {
                    if x <= n {
                        stack.pop();
                        if let Some(&y) = stack.last() {
                            if y > n {
                                sum += x * n;
                            } else {
                                sum += x * y;
                            }
                        } else {
                            sum += x * n;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            stack.push(n);
        }
        while stack.len() >= 2 {
            let n = stack.pop().unwrap();
            sum += stack.last().unwrap() * n;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mct_from_leaf_values() {
        assert_eq!(
            Solution::mct_from_leaf_values(vec![
                5, 1, 2, 3, 15, 7, 3, 2, 2, 5, 9, 1, 11, 9, 15, 14, 7, 1, 5
            ]),
            1166
        );
        assert_eq!(Solution::mct_from_leaf_values(vec![15, 13, 5, 3, 15]), 500);
        assert_eq!(Solution::mct_from_leaf_values(vec![6, 2, 4]), 32);
        assert_eq!(Solution::mct_from_leaf_values(vec![4, 11]), 44);
    }
}
