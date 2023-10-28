// https://leetcode.com/problems/smallest-good-base/description/
// 483. Smallest Good Base
pub struct Solution;
impl Solution {
    fn search_base(n64: u64, l: u32) -> u64 {
        if l == 2 {
            return n64 - 1;
        }
        if (2 << l) - 1 == n64 {
            return 2;
        }
        let mut low = 2u64;
        let mut high = n64 - 1;
        while low + 1 < high {
            let mid = (low + high) / 2;
            let mut sum = 0;
            let mut mul = 1;
            for i in 0..l {
                sum += mul;
                if i + 1 < l {
                    if (n64 - sum) / mul < mid {
                        sum = n64 + 1;
                        break;
                    }
                    mul *= mid;
                }
            }
            if sum == n64 {
                return mid;
            }
            if sum < n64 {
                low = mid;
            } else {
                high = mid;
            }
        }
        0
    }
    pub fn smallest_good_base(n: String) -> String {
        let mut n64 = 0;
        for d in n.bytes() {
            n64 = n64 * 10 + (d - b'0') as u64;
        }
        let k = 64 - n64.leading_zeros();
        for ik in (2..=k).rev() {
            let b = Self::search_base(n64, ik);
            if b != 0 {
                return b.to_string();
            }
        }
        "".to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_good_base() {
        assert_eq!(Solution::smallest_good_base("13".to_string()), "3");
        assert_eq!(Solution::smallest_good_base("4681".to_string()), "8");
        assert_eq!(
            Solution::smallest_good_base("1000000000000000000".to_string()),
            "999999999999999999"
        );
    }
}
