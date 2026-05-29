// https://leetcode.com/problems/minimize-result-by-adding-parentheses-to-expression/
// 2232. Minimize Result by Adding Parentheses to Expression
pub struct Solution;
impl Solution {
    pub fn minimize_result(expression: String) -> String {
        let mut iter = expression.split('+');
        let left = iter.next().unwrap();
        let right = iter.next().unwrap();
        let mut ans = expression.clone();
        let mut min_value = i32::MAX;
        for i in 0..left.len() {
            for j in 1..=right.len() {
                let a = if i == 0 { 1 } else { left[..i].parse::<i32>().unwrap() };
                let b = left[i..].parse::<i32>().unwrap() + right[..j].parse::<i32>().unwrap();
                let c = if j == right.len() {
                    1
                } else {
                    right[j..].parse::<i32>().unwrap()
                };
                let value = a * b * c;
                if value < min_value {
                    min_value = value;
                    ans = format!(
                        "{}({}+{}){}",
                        if i == 0 { "" } else { &left[..i] },
                        &left[i..],
                        &right[..j],
                        if j == right.len() { "" } else { &right[j..] }
                    );
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_result() {
        assert_eq!(Solution::minimize_result("247+38".to_string()), "2(47+38)".to_string());
        assert_eq!(Solution::minimize_result("12+34".to_string()), "1(2+3)4".to_string());
        assert_eq!(
            Solution::minimize_result("999+999".to_string()),
            "(999+999)".to_string()
        );
    }
}
