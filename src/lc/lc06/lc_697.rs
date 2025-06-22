// https://leetcode.com/problems/degree-of-an-array/
// 697. Degree of an Array
pub struct Solution;
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![(0, 0); 50000];
        let mut max_cnt = 0;
        let mut min_len = 0;
        for (i, &n) in nums.iter().enumerate() {
            let n = n as usize;
            cnt[n].0 += 1;
            if cnt[n].0 == 1 {
                cnt[n].1 = i
            }
            if cnt[n].0 > max_cnt {
                max_cnt = cnt[n].0;
                min_len = i - cnt[n].1 + 1;
            } else if cnt[n].0 == max_cnt {
                min_len = min_len.min(i - cnt[n].1 + 1);
            }
        }
        min_len as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_shortest_sub_array() {
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]), 6);
    }
}
