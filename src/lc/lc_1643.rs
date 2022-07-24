pub struct Solution;
impl Solution {
    fn choose(a: i32, b: i32) -> i32 {
        let mut res = 1;
        for i in 1..=b {
            res = res * (a - i + 1) / i;
        }
        res
    }
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let mut nv = destination[0];
        let mut nh = destination[1];
        let mut str = String::new();
        let mut block = Self::choose(nv + nh - 1, nv);
        let mut left = k;
        while left > 0 {
            if left > block {
                str.push('V');
                left -= block;
                nv -= 1;
                block = block * (nv + 1) / (nv + nh);
            } else if left == block {
                str.push('H');
                for _ in 0..nv {
                    str.push('V')
                }
                for _ in 1..nh {
                    str.push('H')
                }
                break;
            } else {
                str.push('H');
                nh -= 1;
                block = block * nh / (nv + nh);
            }
        }
        str
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kth_smallest_path() {
        assert_eq!(Solution::kth_smallest_path(vec![2, 3], 1), "HHHVV");
        assert_eq!(Solution::kth_smallest_path(vec![2, 3], 2), "HHVHV");
        assert_eq!(Solution::kth_smallest_path(vec![2, 3], 3), "HHVVH");
    }
}
