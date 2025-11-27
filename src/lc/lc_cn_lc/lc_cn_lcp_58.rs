// https://leetcode.cn/problems/De4qBB/
// LCP 58. 积木拼接
pub struct Solution;
const FACE_SIDE: [[usize; 4]; 6] = [
    [0, 1, 2, 3],
    [8, 9, 10, 11],
    [0, 4, 8, 5],
    [1, 5, 9, 6],
    [2, 6, 10, 7],
    [3, 7, 11, 4],
];
const FACE_SIDE_DIR: [[i32; 4]; 6] = [
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [-1, 1, 1, -1],
    [-1, 1, 1, -1],
    [-1, 1, 1, -1],
    [-1, 1, 1, -1],
];
const FACE_CORNER: [[usize; 4]; 6] = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [1, 0, 4, 5],
    [2, 1, 5, 6],
    [3, 2, 6, 7],
    [0, 3, 7, 4],
];
struct Cube {
    sides: Vec<Vec<char>>,
    corners: [char; 8],
    cs: [i32; 12],
    cc: [i32; 8],
}
impl Cube {
    fn new(n: usize) -> Self {
        Self {
            sides: vec![vec!['0'; n]; 12],
            corners: ['0'; 8],
            cs: [0; 12],
            cc: [0; 8],
        }
    }
    fn add_shape(&mut self, id: usize, sides: &Vec<Vec<char>>, corners: &[char; 4]) -> bool {
        for i in 0..4 {
            let cid = FACE_CORNER[id][i];
            let sid = FACE_SIDE[id][i];
            if corners[i] == '1' && self.corners[cid] == '1' {
                return false;
            }
            if self.cc[cid] == 2 && corners[i] == '0' && self.corners[cid] == '0' {
                return false;
            }
            let dir = FACE_SIDE_DIR[id][i];
            for j in 0..sides[i].len() {
                let sj = if dir > 0 { j } else { sides[i].len() - 1 - j };
                if sides[i][j] == '1' && self.sides[sid][sj] == '1' {
                    return false;
                }
                if self.cs[sid] == 1 && sides[i][j] == '0' && self.sides[sid][sj] == '0' {
                    return false;
                }
            }
        }
        for i in 0..4 {
            let cid = FACE_CORNER[id][i];
            let sid = FACE_SIDE[id][i];
            if corners[i] == '1' {
                self.corners[cid] = '1';
            }
            let dir = FACE_SIDE_DIR[id][i];
            for j in 0..sides[i].len() {
                let sj = if dir > 0 { j } else { sides[i].len() - 1 - j };
                if sides[i][j] == '1' {
                    self.sides[sid][sj] = '1';
                }
            }
            self.cc[cid] += 1;
            self.cs[sid] += 1;
        }
        true
    }
    fn remove_shape(&mut self, id: usize, sides: &Vec<Vec<char>>, corners: &[char; 4]) {
        for i in 0..4 {
            let cid = FACE_CORNER[id][i];
            let sid = FACE_SIDE[id][i];
            if corners[i] == '1' {
                self.corners[cid] = '0';
            }
            let dir = FACE_SIDE_DIR[id][i];
            for j in 0..sides[i].len() {
                let sj = if dir > 0 { j } else { sides[i].len() - 1 - j };
                if sides[i][j] == '1' {
                    self.sides[sid][sj] = '0';
                }
            }
            self.cc[cid] -= 1;
            self.cs[sid] -= 1;
        }
    }
}
impl Solution {
    pub fn compose_cube(shapes: Vec<Vec<String>>) -> bool {
        let get_sides_corners = |shape: &Vec<String>| -> (Vec<Vec<char>>, [char; 4]) {
            let n = shape[0].len();
            let shape = shape
                .iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            let mut sides = vec![vec![]; 4];
            let corners = [shape[0][0], shape[0][n - 1], shape[n - 1][n - 1], shape[n - 1][0]];
            for i in 1..n - 1 {
                sides[0].push(shape[0][i]);
                sides[1].push(shape[i][n - 1]);
                sides[2].push(shape[n - 1][n - 1 - i]);
                sides[3].push(shape[n - 1 - i][0]);
            }
            (sides, corners)
        };
        let mut side_corners = shapes
            .into_iter()
            .map(|shape| get_sides_corners(&shape))
            .collect::<Vec<(Vec<Vec<char>>, [char; 4])>>();
        fn rotate(side_corner: &mut (Vec<Vec<char>>, [char; 4])) {
            let c = side_corner.1[3];
            side_corner.1[3] = side_corner.1[2];
            side_corner.1[2] = side_corner.1[1];
            side_corner.1[1] = side_corner.1[0];
            side_corner.1[0] = c;
            let s = side_corner.0.pop().unwrap();
            side_corner.0.insert(0, s);
        }
        fn flip(side_corner: &mut (Vec<Vec<char>>, [char; 4])) {
            let c = side_corner.1[0];
            side_corner.1[0] = side_corner.1[1];
            side_corner.1[1] = c;
            let c = side_corner.1[2];
            side_corner.1[2] = side_corner.1[3];
            side_corner.1[3] = c;
            side_corner.0[0].reverse();
            side_corner.0[2].reverse();
            side_corner.0.swap(1, 3);
            side_corner.0[1].reverse();
            side_corner.0[3].reverse();
        }

        fn fill_cube(
            cube: &mut Cube,
            side_cornes: &mut Vec<(Vec<Vec<char>>, [char; 4])>,
            id: usize,
            used: &mut [bool; 6],
        ) -> bool {
            if id == 6 {
                return true;
            }
            if id == 0 {
                cube.add_shape(0, &side_cornes[0].0, &side_cornes[0].1);
                used[0] = true;
                return fill_cube(cube, side_cornes, id + 1, used);
            } else {
                for id_p in 1..6 {
                    if used[id_p] {
                        continue;
                    }
                    for _ in 0..2 {
                        for _ in 0..4 {
                            if cube.add_shape(id, &side_cornes[id_p].0, &side_cornes[id_p].1) {
                                used[id_p] = true;
                                if fill_cube(cube, side_cornes, id + 1, used) {
                                    return true;
                                }
                                used[id_p] = false;
                                cube.remove_shape(id, &side_cornes[id_p].0, &side_cornes[id_p].1);
                            }
                            rotate(&mut side_cornes[id_p]);
                        }
                        flip(&mut side_cornes[id_p]);
                    }
                }
            }
            false
        }
        let mut cube = Cube::new(side_corners[0].0[0].len());
        fill_cube(&mut cube, &mut side_corners, 0, &mut [false; 6])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::compose_cube(vec_vec_str![
                ["1001", "1111", "0111", "1110"],
                ["0000", "0111", "0111", "0101"],
                ["0110", "0110", "1110", "0000"],
                ["1110", "0111", "0110", "0110"],
                ["0011", "0111", "1111", "0001"],
                ["0110", "0111", "0110", "1100"]
            ]),
            true
        );
        assert_eq!(
            Solution::compose_cube(vec_vec_str![
                ["000", "110", "000"],
                ["110", "011", "000"],
                ["110", "011", "110"],
                ["000", "010", "111"],
                ["011", "111", "110"],
                ["110", "010", "000"]
            ]),
            true
        );
        assert_eq!(
            Solution::compose_cube(vec_vec_str![
                ["000", "110", "000"],
                ["110", "011", "000"],
                ["110", "011", "110"],
                ["000", "010", "111"],
                ["011", "111", "011"],
                ["011", "010", "000"]
            ]),
            true
        );
        assert_eq!(
            Solution::compose_cube(vec_vec_str![
                ["101", "111", "000"],
                ["000", "010", "111"],
                ["010", "011", "000"],
                ["010", "111", "010"],
                ["101", "111", "010"],
                ["000", "010", "011"]
            ]),
            false
        );
    }
}
