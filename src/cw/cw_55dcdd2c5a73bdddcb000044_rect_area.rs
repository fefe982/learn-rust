// https://www.codewars.com/kata/55dcdd2c5a73bdddcb000044
// Total area covered by rectangles
use itertools::Itertools;
pub fn calculate(rectangles: &[[i32; 4]]) -> i64 {
    if rectangles.len() == 0 {
        return 0;
    }
    let xl = rectangles
        .iter()
        .map(|r| [r[0], r[2]])
        .flatten()
        .sorted_unstable()
        .unique()
        .collect::<Vec<i32>>();
    let yl = rectangles
        .iter()
        .map(|r| [r[1], r[3]])
        .flatten()
        .sorted_unstable()
        .unique()
        .collect::<Vec<i32>>();
    let mut visited = vec![vec![false; yl.len() - 1]; xl.len() - 1];
    let mut area = 0i64;
    for rect in rectangles {
        let ix0 = xl.binary_search(&rect[0]).unwrap();
        let iy0 = yl.binary_search(&rect[1]).unwrap();
        let ix1 = xl.binary_search(&rect[2]).unwrap();
        let iy1 = yl.binary_search(&rect[3]).unwrap();
        for ix in ix0..ix1 {
            for iy in iy0..iy1 {
                if !visited[ix][iy] {
                    visited[ix][iy] = true;
                    area += (xl[ix + 1] - xl[ix]) as i64 * (yl[iy + 1] - yl[iy]) as i64;
                }
            }
        }
    }
    area
}
#[cfg(test)]
mod sample_tests {
    use super::calculate;

    const ERR_MSG: &str = "\nYour result (left) did not equal expected result (right)";

    #[test]
    fn zero_rectangles() {
        assert_eq!(calculate(&[]), 0, "{}", ERR_MSG);
    }

    #[test]
    fn one_rectangle() {
        assert_eq!(calculate(&[[0, 0, 1, 1]]), 1, "{}", ERR_MSG);
        assert_eq!(calculate(&[[0, 4, 11, 6]]), 22, "{}", ERR_MSG);
    }

    #[test]
    fn two_rectangles() {
        assert_eq!(calculate(&[[0, 0, 1, 1], [1, 1, 2, 2]]), 2, "{}", ERR_MSG);
        assert_eq!(calculate(&[[0, 0, 1, 1], [0, 0, 2, 2]]), 4, "{}", ERR_MSG);
    }

    #[test]
    fn three_rectangles() {
        assert_eq!(
            calculate(&[[3, 3, 8, 5], [6, 3, 8, 9], [11, 6, 14, 12]]),
            36,
            "{}",
            ERR_MSG
        );
    }
}
