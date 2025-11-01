// https://leetcode.com/problems/count-binary-palindromic-numbers/
// 3677. Count Binary Palindromic Numbers
pub struct Solution;
impl Solution {
    pub fn count_binary_palindromes(n: i64) -> i32 {
        if n == 0 {
            return 1;
        }
        if n <= 2 {
            return 2;
        }
        let nbit = i64::BITS - n.leading_zeros();
        let mut count = 0;
        let mut m = 0;
        for bit in nbit / 2..nbit {
            if n & (1 << bit) != 0 {
                m |= 1 << bit;
                m |= 1 << (nbit - 1 - bit);
            }
        }
        if m <= n {
            count += 1;
        }
        count += (n ^ (1 << (nbit - 1))) >> (nbit / 2);
        for bit in 1..nbit {
            count += 1 << ((bit - 1) / 2);
        }
        (count + 1) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_binary_palindromes() {
        assert_eq!(Solution::count_binary_palindromes(9), 6);
        assert_eq!(Solution::count_binary_palindromes(0), 1);
    }
}
