// https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/
// 331. Verify Preorder Serialization of a Binary Tree
pub struct Solution;
impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut stack = vec![0];
        for p in preorder.split(",") {
            if let Some(state) = stack.pop() {
                if state == 0 {
                    if p == "#" {
                        continue;
                    }
                    stack.push(1);
                    stack.push(0);
                } else if state == 1 {
                    if p == "#" {
                        continue;
                    } else {
                        stack.push(1);
                        stack.push(0);
                    }
                }
            } else {
                return false;
            }
        }
        return stack.is_empty();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid_serialization() {
        assert_eq!(
            Solution::is_valid_serialization(String::from("9,3,4,#,#,1,#,#,2,#,6,#,#")),
            true
        );
        assert_eq!(Solution::is_valid_serialization(String::from("1,#")), false);
        assert_eq!(Solution::is_valid_serialization(String::from("9,#,#,1")), false);
    }
}
