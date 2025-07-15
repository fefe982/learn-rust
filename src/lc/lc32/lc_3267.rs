// https://leetcode.com/problems/count-almost-equal-pairs-ii/
// 3267. Count Almost Equal Pairs II
pub struct Solution;
impl Solution {
    pub fn count_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut nx = vec![0; 4];
        let mut ny = vec![0; 4];
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                let mut x = nums[i];
                let mut y = nums[j];
                let mut diff = 0;
                for _ in 0..7 {
                    if x % 10 != y % 10 {
                        if diff < 4 {
                            nx[diff] = x % 10;
                            ny[diff] = y % 10;
                        }
                        diff += 1;
                        if diff > 4 {
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
                } else if diff == 3 {
                    if (nx[0] == ny[1] && nx[1] == ny[2] && nx[2] == ny[0])
                        || (nx[0] == ny[2] && nx[1] == ny[0] && nx[2] == ny[1])
                    {
                        cnt += 1;
                    }
                } else if diff == 4 {
                    if (nx[0] == ny[1] && nx[1] == ny[0] && nx[2] == ny[3] && nx[3] == ny[2])
                        || (nx[0] == ny[2] && nx[2] == ny[0] && nx[1] == ny[3] && nx[3] == ny[1])
                        || (nx[0] == ny[3] && nx[3] == ny[0] && nx[1] == ny[2] && nx[2] == ny[1])
                    {
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
        assert_eq!(Solution::count_pairs(vec![1023, 2310, 2130, 213]), 4);
        assert_eq!(Solution::count_pairs(vec![1, 10, 100]), 3);
    }
}
