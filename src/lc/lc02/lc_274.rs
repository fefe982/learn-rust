// https://leetcode.com/problems/h-index/
// 274. H-Index
pub struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut c = citations;
        c.sort_unstable();
        let mut i = 0;
        let l = c.len();
        let mut j = l - 1;
        if c[j] < 1 {
            return 0;
        }
        if c[0] >= l as i32 {
            return l as i32;
        }
        while i + 1 < j {
            let mid = (i + j) / 2;
            if c[mid] >= (l - mid) as i32 {
                j = mid;
            } else {
                i = mid;
            }
        }
        (l - j) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_h_index() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }
}
