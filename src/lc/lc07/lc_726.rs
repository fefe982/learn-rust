// https://leetcode.com/problems/number-of-atoms/
// 726. Number of Atoms
pub struct Solution;
impl Solution {
    fn push(
        stack: &mut Vec<std::collections::BTreeMap<String, i32>>,
        ele: String,
        tree: Option<std::collections::BTreeMap<String, i32>>,
        count: i32,
    ) {
        let cnt = if count == 0 { 1 } else { count };
        if !ele.is_empty() {
            *(stack.last_mut().unwrap().entry(ele).or_default()) += cnt;
        }
        if let Some(t) = tree {
            for (e, c) in t {
                *(stack.last_mut().unwrap().entry(e).or_default()) += c * cnt;
            }
        }
    }
    pub fn count_of_atoms(formula: String) -> String {
        let mut stack = vec![std::collections::BTreeMap::<String, i32>::new()];
        let mut tree = None;
        let mut ele = "".to_string();
        let mut count = 0;
        for c in formula.chars().chain("Z".chars()) {
            match c {
                'A'..='Z' => {
                    Solution::push(&mut stack, ele, tree.take(), count);
                    ele = c.to_string();
                    count = 0;
                }
                'a'..='z' => ele.push(c),
                '0'..='9' => count = count * 10 + (c as i32 - '0' as i32),
                '(' => {
                    Solution::push(&mut stack, ele, tree.take(), count);
                    ele = "".to_string();
                    count = 0;
                    stack.push(std::collections::BTreeMap::<String, i32>::new());
                }
                ')' => {
                    Solution::push(&mut stack, ele, tree.take(), count);
                    ele = "".to_string();
                    count = 0;
                    tree = Some(stack.pop().unwrap());
                }
                _ => unreachable!(),
            }
        }
        let mut ans = "".to_string();
        for (e, c) in stack.pop().unwrap().into_iter() {
            ans.push_str(&e);
            if c > 1 {
                ans.push_str(&c.to_string());
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_of_atoms() {
        assert_eq!(Solution::count_of_atoms("H2O".to_string()), "H2O".to_string());
        assert_eq!(Solution::count_of_atoms("Mg(OH)2".to_string()), "H2MgO2".to_string());
        assert_eq!(
            Solution::count_of_atoms("K4(ON(SO3)2)2".to_string()),
            "K4N2O14S4".to_string()
        );
    }
}
