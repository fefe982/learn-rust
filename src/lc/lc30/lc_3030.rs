// https://leetcode.com/problems/find-the-grid-of-region-average/
// 3030. Find the Grid of Region Average Value
pub struct Solution;
impl Solution {
    pub fn result_grid(image: Vec<Vec<i32>>, threshold: i32) -> Vec<Vec<i32>> {
        fn get_raw_value(x: i32) -> i32 {
            x & 0xff
        }
        fn get_cnt(x: i32) -> i32 {
            (x >> 8) & 0xf
        }
        fn set_cnt(x: i32, cnt: i32) -> i32 {
            (x & !0xf00) | (cnt << 8)
        }
        fn get_sum(x: i32) -> i32 {
            x >> 12
        }
        fn set_sum(x: i32, sum: i32) -> i32 {
            (x & 0xfff) | (sum << 12)
        }
        let mut image = image;
        for i in 0..image.len() - 2 {
            for j in 0..image[0].len() - 2 {
                let mut sum = 0;
                let mut isregion = true;
                for ii in 0..3 {
                    for jj in 0..3 {
                        let nij = get_raw_value(image[i + ii][j + jj]);
                        sum += nij;
                        if ii != 2 && (nij - get_raw_value(image[i + ii + 1][j + jj])).abs() > threshold {
                            isregion = false;
                            break;
                        }
                        if jj != 2 && (nij - get_raw_value(image[i + ii][j + jj + 1])).abs() > threshold {
                            isregion = false;
                        }
                    }
                    if !isregion {
                        break;
                    }
                }
                if isregion {
                    for ii in 0..3 {
                        for jj in 0..3 {
                            image[i + ii][j + jj] =
                                set_sum(image[i + ii][j + jj], get_sum(image[i + ii][j + jj]) + sum / 9);
                            image[i + ii][j + jj] = set_cnt(image[i + ii][j + jj], get_cnt(image[i + ii][j + jj]) + 1);
                        }
                    }
                }
            }
        }
        for i in 0..image.len() {
            for j in 0..image[0].len() {
                if get_cnt(image[i][j]) != 0 {
                    image[i][j] = get_sum(image[i][j]) / get_cnt(image[i][j])
                }
            }
        }
        image
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn result_grid() {
        assert_eq!(
            Solution::result_grid(
                vec_vec![
                    [0, 20, 41, 64, 47],
                    [48, 50, 75, 0, 9],
                    [13, 66, 15, 61, 22],
                    [44, 61, 37, 25, 80],
                    [31, 62, 50, 63, 31]
                ],
                81
            ),
            vec_vec![
                [36, 39, 38, 40, 37],
                [40, 41, 40, 39, 36],
                [41, 42, 41, 41, 38],
                [43, 44, 42, 42, 39],
                [42, 45, 44, 45, 42]
            ]
        );
        assert_eq!(
            Solution::result_grid(vec_vec![[5, 6, 7, 10], [8, 9, 10, 10], [11, 12, 13, 10]], 3),
            vec_vec![[9, 9, 9, 9], [9, 9, 9, 9], [9, 9, 9, 9]]
        );
        assert_eq!(
            Solution::result_grid(vec_vec![[10, 20, 30], [15, 25, 35], [20, 30, 40], [25, 35, 45]], 12),
            vec_vec![[25, 25, 25], [27, 27, 27], [27, 27, 27], [30, 30, 30]]
        );
        assert_eq!(
            Solution::result_grid(vec_vec![[5, 6, 7], [8, 9, 10], [11, 12, 13]], 1),
            vec_vec![[5, 6, 7], [8, 9, 10], [11, 12, 13]]
        );
    }
}
