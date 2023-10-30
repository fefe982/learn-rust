// https://leetcode.com/problems/h-index-ii/
// 275. H-Index II
pub struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut i = 0;
        let l = citations.len();
        let mut j = l - 1;
        if citations[j] < 1 {
            return 0;
        }
        if citations[0] >= l as i32 {
            return l as i32;
        }
        while i + 1 < j {
            let mid = (i + j) / 2;
            if citations[mid] >= (l - mid) as i32 {
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
        assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
        assert_eq!(Solution::h_index(vec![1, 2, 100]), 2);
    }
}
