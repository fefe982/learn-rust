// https://leetcode.com/problems/cracking-the-safe/
// 753. Cracking the Safe
pub struct Solution;
impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let mut m = std::collections::HashSet::<String>::new();
        let mut res = String::new();
        let total = (1..n).fold(k, |acc, _| acc * k);
        let mut stack_next = vec![];
        let mut next_cur = 0;
        let n = n as usize;
        for _ in 1..n {
            res.push('0');
        }
        'main: loop {
            let mut pre = res[res.len() + 1 - n..].to_owned();
            while next_cur < k {
                pre.push(('0' as u8 + next_cur as u8) as char);
                next_cur += 1;
                if m.contains(&pre) {
                    pre.pop();
                } else {
                    break;
                }
                if next_cur >= k {
                    m.remove(&res[res.len() - n..].to_owned());
                    next_cur = stack_next.pop().unwrap();
                    res.pop();
                    continue 'main;
                }
            }
            res.push(('0' as u8 + next_cur as u8 - 1) as char);
            m.insert(pre);
            if m.len() == total as usize {
                break;
            }
            stack_next.push(next_cur);
            next_cur = 0;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_crack_safe() {
        assert_eq!(Solution::crack_safe(1, 2), "01");
        assert_eq!(Solution::crack_safe(2, 2), "00110");
    }
}
