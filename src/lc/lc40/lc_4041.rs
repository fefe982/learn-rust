// https://leetcode.com/problems/minimum-operations-to-form-subset-sum-ii/
// 4041. Minimum Operations to Form Subset Sum II
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, sum: i32) -> i32 {
        let mut cost = vec![i32::MAX; sum as usize + 1];
        cost[0] = 0;
        for &n in &nums {
            for i in (1..=sum).rev() {
                let mut d = 0;
                let mut odd = 1;
                let mut dn = n;
                while dn > i {
                    d += 1;
                    dn /= 2;
                }
                while dn > 0 {
                    if cost[(i - dn) as usize] != i32::MAX {
                        cost[i as usize] = cost[i as usize].min(cost[(i - dn) as usize] + d);
                    }
                    if odd == 1 {
                        let mut u = 1;
                        let mut un = dn * 2;
                        while un <= i {
                            if cost[(i - un) as usize] != i32::MAX {
                                cost[i as usize] = cost[i as usize].min(cost[(i - un) as usize] + d + u);
                            }
                            u += 1;
                            un *= 2;
                        }
                    }
                    odd = dn % 2;
                    dn /= 2;
                    d += 1;
                }
            }
        }
        if cost[sum as usize] == i32::MAX {
            -1
        } else {
            cost[sum as usize]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![10, 2], 13), 3);
        assert_eq!(Solution::min_operations(vec![6, 3], 8), 2);
        assert_eq!(Solution::min_operations(vec![2, 2], 7), -1);
    }
}
