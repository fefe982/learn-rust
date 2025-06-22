// https://leetcode.com/problems/magical-string/
// 481. Magical String
pub struct Solution;
impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n <= 3 {
            return 1;
        }
        let mut q = std::collections::VecDeque::new();
        let mut cnt = 1;
        let mut last = 2;
        let mut n = n;
        n -= 3;
        q.push_back(2);
        while n > 0 {
            let k = q.pop_front().unwrap();
            last = 3 - last;
            if last == 1 {
                cnt += n.min(k);
            }
            n -= k;
            q.push_back(last);
            if k > 1 {
                q.push_back(last);
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn magical_string() {
        assert_eq!(Solution::magical_string(6), 3);
    }
}
