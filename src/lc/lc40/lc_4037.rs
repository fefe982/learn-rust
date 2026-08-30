// https://leetcode.com/problems/maximum-valid-split-positions-ii/
// 4037. Maximum Valid Split Positions II
pub struct Solution;
impl Solution {
    pub fn max_valid_splits(nums: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let mut gcdl = vec![0; nums.len()];
        let mut gcdr = vec![0; nums.len()];
        gcdl[0] = nums[0];
        let mut il = 0;
        for i in 1..nums.len() {
            gcdl[i] = gcd(gcdl[i - 1], nums[i]);
            if gcdl[i] != gcdl[i - 1] {
                il = i;
            }
        }
        gcdr[nums.len() - 1] = nums[nums.len() - 1];
        let mut ir = nums.len() - 1;
        for i in (0..nums.len() - 1).rev() {
            gcdr[i] = gcd(gcdr[i + 1], nums[i]);
            if gcdr[i] != gcdr[i + 1] {
                ir = i;
            }
        }
        if ir > il {
            return (ir - il) as i32;
        }
        let mut max = 0;
        if ir == 0 {
            let mut nir = 1;
            let mut g = nums[1];
            let mut nil = if g == gcdr[1] { 1 } else { usize::MAX };
            for i in 2..nums.len() {
                if gcdr[i] != gcdr[1] {
                    break;
                }
                if nil == usize::MAX {
                    g = gcd(g, nums[i]);
                    if g == gcdr[1] {
                        nil = i;
                    }
                }
                nir = i;
            }
            if nil != usize::MAX {
                max = (nir - nil) as i32;
            }
        }
        if il == nums.len() - 1 {
            let mut nil = nums.len() - 2;
            let mut g = nums[nums.len() - 2];
            let mut nir = if g == gcdl[nums.len() - 2] {
                nums.len() - 2
            } else {
                usize::MAX
            };
            for i in (0..nums.len() - 2).rev() {
                if gcdl[i] != gcdl[nums.len() - 2] {
                    break;
                }
                if nir == usize::MAX {
                    g = gcd(g, nums[i]);
                    if g == gcdl[nums.len() - 2] {
                        nir = i;
                    }
                }
                nil = i;
            }
            if nir != usize::MAX {
                max = (nir - nil) as i32;
            }
        }
        for i in ir.max(1)..=il.min(nums.len() - 2) {
            if gcdl[i - 1] == gcdr[i + 1] {
                let mut nil = i - 1;
                while nil > 0 && gcdl[nil - 1] == gcdl[i - 1] {
                    nil -= 1;
                }
                let mut nir = i + 1;
                while nir < nums.len() - 1 && gcdr[nir + 1] == gcdr[i + 1] {
                    nir += 1;
                }
                max = max.max((nir - nil) as i32 - 1);
            }
            if gcdl[i - 1] > gcdr[i + 1] && gcdl[i - 1] > gcdl[i] {
                let mut g = gcdl[i - 1];
                let mut nil = usize::MAX;
                let mut nir = i + 1;
                for j in i + 1..nums.len() {
                    if gcdr[j] > gcdr[i + 1] {
                        break;
                    }
                    nir = j;
                    if nil == usize::MAX {
                        g = gcd(g, nums[j]);
                        if g < gcdr[i + 1] {
                            break;
                        }
                        if g == gcdr[i + 1] {
                            nil = j;
                        }
                    }
                }
                if nil != usize::MAX {
                    max = max.max((nir - nil) as i32);
                }
            }
            if gcdr[i + 1] > gcdl[i - 1] && gcdr[i + 1] > gcdr[i] {
                let mut g = gcdr[i + 1];
                let mut nir = usize::MAX;
                let mut nil = i - 1;
                for j in (0..i).rev() {
                    if gcdl[j] > gcdl[i - 1] {
                        break;
                    }
                    nil = j;
                    if nir == usize::MAX {
                        g = gcd(g, nums[j]);
                        if g < gcdl[i - 1] {
                            break;
                        }
                        if g == gcdl[i - 1] {
                            nir = j;
                        }
                    }
                }
                if nir != usize::MAX {
                    max = max.max((nir - nil) as i32);
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_valid_splits() {
        assert_eq!(Solution::max_valid_splits(vec![6, 10, 10, 1, 8]), 1);
        assert_eq!(Solution::max_valid_splits(vec![10, 6, 6]), 1);
        assert_eq!(Solution::max_valid_splits(vec![10, 30, 15, 10]), 2);
        assert_eq!(Solution::max_valid_splits(vec![2, 10, 14]), 1);
        assert_eq!(Solution::max_valid_splits(vec![2, 4]), 0);
    }
}
