// https://leetcode.com/problems/find-a-peak-element-ii/
// 1901. Find a Peak Element II
pub struct Solution;
impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut maxi_low, max_low) =
            mat[0].iter().enumerate().fold(
                (0, i32::MIN),
                |(maxi, max), (i, &x)| if max > x { (maxi, max) } else { (i, x) },
            );
        let mut low = 0;
        if mat.len() == 1 || mat[1][maxi_low] < max_low {
            return vec![0, maxi_low as i32];
        }
        let m = mat.len();
        let (mut maxi_high, mut max_high) = mat[m - 1].iter().enumerate().fold(
            (0, i32::MIN),
            |(maxi, max), (i, &x)| if max > x { (maxi, max) } else { (i, x) },
        );
        let mut high = m - 1;
        if mat[high - 1][maxi_high] < max_high {
            return vec![high as i32, maxi_high as i32];
        }
        while low + 1 < high {
            let mid = low + (high - low) / 2;
            let (maxi_mid, max_mid) =
                mat[mid].iter().enumerate().fold(
                    (0, i32::MIN),
                    |(maxi, max), (i, &x)| if max > x { (maxi, max) } else { (i, x) },
                );
            if mat[mid - 1][maxi_mid] < max_mid && mat[mid + 1][maxi_mid] < max_mid {
                return vec![mid as i32, maxi_mid as i32];
            }
            if mat[mid - 1][maxi_mid] < max_mid {
                low = mid;
                maxi_low = maxi_mid;
            } else {
                high = mid - 1;
                if high == low {
                    return vec![low as i32, maxi_low as i32];
                }
                (maxi_high, max_high) =
                    mat[high].iter().enumerate().fold(
                        (0, i32::MIN),
                        |(maxi, max), (i, &x)| if max > x { (maxi, max) } else { (i, x) },
                    );
                if mat[high - 1][maxi_high] < max_high {
                    return vec![high as i32, maxi_high as i32];
                }
            }
        }
        vec![low as i32, maxi_low as i32]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_peak_grid() {
        assert_eq!(Solution::find_peak_grid(vec_vec![[1, 4], [3, 2]]), vec![0, 1]);
        assert_eq!(
            Solution::find_peak_grid(vec_vec![[10, 20, 15], [21, 30, 14], [7, 16, 32]]),
            vec![2, 2]
        );
    }
}
