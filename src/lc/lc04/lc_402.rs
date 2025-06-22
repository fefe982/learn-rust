// https://leetcode.com/problems/remove-k-digits/
// 402. Remove K Digits
pub struct Solution;
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut h = std::collections::BinaryHeap::<std::cmp::Reverse<(char, usize)>>::new();
        let mut to_remove = k as usize;
        let mut ans = "".to_string();
        let mut last = 0;
        let num = num.chars().collect::<Vec<_>>();
        for (&c, i) in num.iter().zip(1..) {
            h.push(std::cmp::Reverse((c, i)));
            if i == last + to_remove + 1 {
                let std::cmp::Reverse((nc, ni)) = h.pop().unwrap();
                if !ans.is_empty() || nc != '0' {
                    ans.push(nc);
                }
                to_remove -= ni - last - 1;
                last = ni;
                while let Some(&std::cmp::Reverse((_, ii))) = h.peek() {
                    if ii <= last {
                        h.pop();
                    } else {
                        break;
                    }
                }
            }
        }
        to_remove -= num.len() - last;
        to_remove = to_remove.min(ans.len());
        while to_remove > 0 {
            ans.pop();
            to_remove -= 1;
        }
        if ans.is_empty() {
            "0".to_string()
        } else {
            ans
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_kdigits() {
        assert_eq!(Solution::remove_kdigits("112".to_string(), 1), "11");
        assert_eq!(Solution::remove_kdigits("1432219".to_string(), 3), "1219");
        assert_eq!(Solution::remove_kdigits("10200".to_string(), 1), "200");
        assert_eq!(Solution::remove_kdigits("10".to_string(), 2), "0");
    }
}
