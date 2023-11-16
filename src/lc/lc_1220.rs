// https://leetcode.com/problems/count-vowels-permutation/
// 1220. Count Vowels Permutation
pub struct Solution;
const MOD: i64 = 1_000_000_007;
#[derive(Copy, Clone)]
struct IMod {
    val: i64,
}
impl IMod {
    fn from_i32(val: i32) -> Self {
        Self { val: val as i64 }
    }
    fn to_i32(&self) -> i32 {
        self.val as i32
    }
}
impl std::ops::Add for IMod {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            val: (self.val + rhs.val) % MOD,
        }
    }
}
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut cnt = vec![IMod::from_i32(1); 5];
        for _ in 2..=n {
            let mut next = vec![IMod::from_i32(0); 5];
            next[0] = cnt[1] + cnt[2] + cnt[4];
            next[1] = cnt[0] + cnt[2];
            next[2] = cnt[1] + cnt[3];
            next[3] = cnt[2];
            next[4] = cnt[2] + cnt[3];
            cnt = next;
        }
        cnt.into_iter()
            .fold(IMod::from_i32(0), |acc, x| acc + x)
            .to_i32()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_vowel_permutation() {
        assert_eq!(Solution::count_vowel_permutation(1), 5);
        assert_eq!(Solution::count_vowel_permutation(2), 10);
        assert_eq!(Solution::count_vowel_permutation(5), 68);
    }
}
