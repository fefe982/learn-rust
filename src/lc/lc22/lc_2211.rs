// https://leetcode.com/problems/count-collisions-on-a-road/
// 2211. Count Collisions on a Road
pub struct Solution;
impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut stk = vec![];
        let mut ans = 0;
        for mut d in directions.chars() {
            if d == 'R' {
                stk.push(d);
                continue;
            }
            if d == 'L' {
                if let Some(&top) = stk.last() {
                    if top == 'R' {
                        stk.pop();
                        ans += 2;
                        d = 'S';
                    } else if top == 'S' {
                        ans += 1;
                        continue;
                    } else {
                        continue;
                    }
                }
            }
            if d == 'S' {
                while let Some(&top) = stk.last() {
                    if top == 'R' {
                        stk.pop();
                        ans += 1;
                    } else {
                        break;
                    }
                }
                stk.push(d);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_collisions() {
        assert_eq!(Solution::count_collisions("RLRSLL".to_string()), 5);
        assert_eq!(Solution::count_collisions("LLRR".to_string()), 0);
    }
}
