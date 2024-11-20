// https://leetcode.com/problems/count-almost-equal-pairs-i
// 3265. Count Almost Equal Pairs I
pub struct Solution;
impl Solution {
    pub fn count_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut nx = vec![0; 2];
        let mut ny = vec![0; 2];
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                let mut x = nums[i];
                let mut y = nums[j];
                let mut diff = 0;
                for _ in 0..7 {
                    if x % 10 != y % 10 {
                        if diff < 2 {
                            nx[diff] = x % 10;
                            ny[diff] = y % 10;
                        }
                        diff += 1;
                        if diff > 2 {
                            break;
                        }
                    }
                    x /= 10;
                    y /= 10;
                }
                if diff == 0 {
                    cnt += 1;
                } else if diff == 2 {
                    if nx[0] == ny[1] && nx[1] == ny[0] {
                        cnt += 1;
                    }
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_pairs() {
        assert_eq!(Solution::count_pairs(vec![3, 12, 30, 17, 21]), 2);
        assert_eq!(Solution::count_pairs(vec![1, 1, 1, 1, 1]), 10);
        assert_eq!(Solution::count_pairs(vec![123, 231]), 0);
    }
}
