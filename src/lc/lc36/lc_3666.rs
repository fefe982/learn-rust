// https://leetcode.com/problems/minimum-operations-to-equalize-binary-string/
// 3666. Minimum Operations to Equalize Binary String
pub struct Solution;
impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let mut z = 0;
        let mut nz = 0;
        for c in s.chars() {
            match c {
                '0' => z += 1,
                '1' => nz += 1,
                _ => {}
            }
        }
        if z == 0 {
            return 0;
        }
        let n = z + nz;
        if n == k {
            if z == n {
                return 1;
            } else {
                return -1;
            }
        }
        let nk = n - k;
        let mut ne = i32::MAX;
        let zkk = (z + k - 1) / k;
        if z % 2 == 0 {
            ne = zkk.max((z + nk - 1) / nk);
            ne += ne % 2;
        }
        let mut no = i32::MAX;
        if z % 2 == k % 2 {
            no = zkk.max((nz + nk - 1) / nk);
            no |= 1;
        }
        let nm = ne.min(no);
        if nm == i32::MAX {
            -1
        } else {
            nm
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations("110".to_string(), 1), 1);
        assert_eq!(Solution::min_operations("0101".to_string(), 3), 2);
        assert_eq!(Solution::min_operations("101".to_string(), 2), -1);
    }
}
