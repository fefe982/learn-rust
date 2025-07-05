// https://leetcode.com/problems/binary-trees-with-factors/
// 823. Binary Trees With Factors
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
impl std::ops::Mul<IMod> for IMod {
    type Output = IMod;
    fn mul(self, rhs: IMod) -> IMod {
        IMod {
            val: (self.val * rhs.val) % MOD,
        }
    }
}
impl std::ops::Mul<i32> for IMod {
    type Output = IMod;
    fn mul(self, rhs: i32) -> IMod {
        IMod {
            val: (self.val * rhs as i64) % MOD,
        }
    }
}
impl std::ops::AddAssign for IMod {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val) % MOD;
    }
}
impl std::ops::Add<IMod> for IMod {
    type Output = IMod;
    fn add(self, rhs: IMod) -> IMod {
        IMod {
            val: (self.val + rhs.val) % MOD,
        }
    }
}
impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort();
        let mut dp = vec![IMod::from_i32(1); arr.len()];
        let mut sum = dp[0];
        for i in 1..arr.len() {
            let mut j = 0;
            let mut k = i - 1;
            let (left, right) = dp.split_at_mut(i);
            while j <= k {
                match (arr[i] % arr[j] == 0, (arr[i] / arr[j]).cmp(&arr[k])) {
                    (true, std::cmp::Ordering::Equal) => {
                        if j == k {
                            right[0] += left[j] * left[k];
                        } else {
                            right[0] += left[j] * left[k] * 2;
                        }
                        j += 1;
                        if k == 0 {
                            break;
                        }
                        k -= 1;
                    }
                    (_, std::cmp::Ordering::Greater) => j += 1,
                    (_, std::cmp::Ordering::Less) => {
                        if k == 0 {
                            break;
                        }
                        k -= 1;
                    }
                    (false, std::cmp::Ordering::Equal) => j += 1,
                }
            }
            sum += right[0]
        }
        sum.to_i32()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_factored_binary_trees() {
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 4]), 3);
        assert_eq!(Solution::num_factored_binary_trees(vec![2, 4, 5, 10]), 7);
    }
}
