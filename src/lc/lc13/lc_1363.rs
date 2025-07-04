// https://leetcode.com/problems/largest-multiple-of-three/
// 1363. Largest Multiple of Three
pub struct Solution;
impl Solution {
    fn remove_one(cnt: &mut [i32]) -> bool {
        if cnt[1] > 0 {
            cnt[1] -= 1;
            true
        } else if cnt[4] > 0 {
            cnt[4] -= 1;
            true
        } else if cnt[7] > 0 {
            cnt[7] -= 1;
            true
        } else {
            false
        }
    }
    fn remove_two(cnt: &mut [i32]) -> bool {
        if cnt[2] > 0 {
            cnt[2] -= 1;
            true
        } else if cnt[5] > 0 {
            cnt[5] -= 1;
            true
        } else if cnt[8] > 0 {
            cnt[8] -= 1;
            true
        } else {
            false
        }
    }
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut cnt = vec![0; 10];
        let mut sum = 0;
        for d in digits {
            cnt[d as usize] += 1;
            sum += d;
        }
        if sum % 3 == 1 {
            if !Self::remove_one(&mut cnt) {
                Self::remove_two(&mut cnt);
                Self::remove_two(&mut cnt);
            }
        } else if sum % 3 == 2 {
            if !Self::remove_two(&mut cnt) {
                Self::remove_one(&mut cnt);
                Self::remove_one(&mut cnt);
            }
        }
        let mut ans = String::new();
        for i in (0..10).rev() {
            if cnt[i] > 0 {
                if i == 0 && ans.is_empty() {
                    return "0".to_string();
                }
                ans.push_str(i.to_string().repeat(cnt[i] as usize).as_str());
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest_multiple_of_three() {
        assert_eq!(Solution::largest_multiple_of_three(vec![1, 1, 1, 2]), "111");
        assert_eq!(Solution::largest_multiple_of_three(vec![8, 1, 9]), "981");
        assert_eq!(Solution::largest_multiple_of_three(vec![8, 6, 7, 1, 0]), "8760");
        assert_eq!(Solution::largest_multiple_of_three(vec![1]), "");
    }
}
