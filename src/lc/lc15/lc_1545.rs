// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/
// 1545. Find Kth Bit in Nth Binary String
pub struct Solution;
impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }
        let mid = 1 << (n - 1);
        match k.cmp(&mid) {
            std::cmp::Ordering::Equal => '1',
            std::cmp::Ordering::Less => Solution::find_kth_bit(n - 1, k),
            std::cmp::Ordering::Greater => {
                if Solution::find_kth_bit(n - 1, mid * 2 - k) == '0' {
                    '1'
                } else {
                    '0'
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_kth_bit() {
        assert_eq!(Solution::find_kth_bit(3, 1), '0');
        assert_eq!(Solution::find_kth_bit(4, 11), '1');
    }
}
