// https://leetcode.com/problems/letter-tile-possibilities/
// 1079. Letter Tile Possibilities
pub struct Solution;
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let tiles = tiles.as_bytes();
        let l = tiles.len();
        let mut vec = vec![0; l + 1];
        vec[0] = 1;
        let mut cnt = vec![0; 26];
        for &t in tiles {
            cnt[(t - b'A') as usize] += 1;
        }
        for c in cnt {
            if c == 0 {
                continue;
            }
            let mut nvec = vec![0; l + 1];
            nvec[0] = 1;
            for il in 0..=l {
                let mut s = 1;
                for ic in 1..=c {
                    if vec[il] == 0 {
                        break;
                    }
                    s = s * (il + ic) / ic;
                    nvec[ic + il as usize] += vec[il] * s;
                }
            }
            for il in 1..=l {
                vec[il] += nvec[il];
            }
        }
        vec.into_iter().sum::<usize>() as i32 - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_tile_possibilities() {
        assert_eq!(Solution::num_tile_possibilities(String::from("AAB")), 8);
        assert_eq!(
            Solution::num_tile_possibilities(String::from("AAABBC")),
            188
        );
        assert_eq!(Solution::num_tile_possibilities(String::from("V")), 1);
    }
}
