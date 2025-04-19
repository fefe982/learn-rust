// https://leetcode.com/problems/rabbits-in-forest/
// 781. Rabbits in Forest
pub struct Solution;
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut cnt = [0; 1000];
        let mut res = 0;
        for &a in answers.iter() {
            let a = a as usize;
            if cnt[a] % (a + 1) == 0 {
                res += a as i32 + 1;
            }
            cnt[a] += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_rabbits() {
        assert_eq!(Solution::num_rabbits(vec![0, 0, 1, 1, 1]), 6);
        assert_eq!(Solution::num_rabbits(vec![1, 0, 1, 0, 0]), 5);
        assert_eq!(Solution::num_rabbits(vec![1, 1, 2]), 5);
        assert_eq!(Solution::num_rabbits(vec![10, 10, 10]), 11);
    }
}
