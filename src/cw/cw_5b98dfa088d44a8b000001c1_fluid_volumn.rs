// https://www.codewars.com/kata/5b98dfa088d44a8b000001c1
// Fluid Volume of a Heightmap
fn volume(heightmap: &Vec<Vec<i32>>) -> i32 {
    let mut heightmap = heightmap.clone();
    let h = heightmap.len();
    let w = heightmap[0].len();
    let mut fixed = vec![vec![false; w]; h];
    for i in 0..h {
        fixed[i][0] = true;
        fixed[i][w - 1] = true;
    }
    for i in 0..w {
        fixed[0][i] = true;
        fixed[h - 1][i] = true;
    }
    let mut nwater = 0;
    for ih in 1..h - 1 {
        for iw in 1..w - 1 {
            if fixed[ih][iw] {
                continue;
            }
            let mut visited = vec![vec![false; w]; h];
            let mut q = std::collections::BinaryHeap::new();
            q.push((-heightmap[ih][iw], ih, iw));
            visited[ih][iw] = true;
            let mut min_bound = heightmap[ih][iw];
            let mut filled = vec![];
            let mut fix = false;
            while let Some((h, ch, cw)) = q.pop() {
                min_bound = -h;
                if fixed[ch][cw] {
                    fix = true;
                    break;
                }
                filled.push((ch, cw));
                let mut rise = true;
                for d in [[0, -1], [-1, 0], [1, 0], [0, 1]] {
                    let nh = (ch as i32 + d[0]) as usize;
                    let nw = (cw as i32 + d[1]) as usize;
                    if !visited[nh][nw] && heightmap[nh][nw] < heightmap[ch][cw] {
                        rise = false;
                        break;
                    }
                }
                if !rise {
                    break;
                }
                for d in [[0, -1], [-1, 0], [1, 0], [0, 1]] {
                    let nh = (ch as i32 + d[0]) as usize;
                    let nw = (cw as i32 + d[1]) as usize;
                    if !visited[nh][nw] {
                        visited[nh][nw] = true;
                        q.push((-heightmap[nh][nw], nh, nw));
                    }
                }
            }
            for (ch, cw) in filled {
                nwater += min_bound - heightmap[ch][cw];
                heightmap[ch][cw] = min_bound;
                if fix {
                    fixed[ch][cw] = true;
                }
            }
        }
    }
    nwater
}
// Add your own tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    // this just helps with the test output on failure.
    fn pretty_test(map: &Vec<Vec<i32>>, expected: i32) {
        let result = volume(&map);
        let mut printy = String::new();
        for row in map {
            printy.push_str(format!("{:?}\n", row).as_str());
        }
        assert_eq!(
            result, expected,
            "\nYour result (left) did not match expected result (right) for map:\n{}",
            printy
        );
    }

    #[test]
    fn small_maps_test() {
        let tests = [
            (vec![vec![0]], 0),
            (vec![vec![22]], 0),
            (vec![vec![2, 1, 2], vec![1, 0, 1], vec![2, 1, 2]], 1),
            (vec![vec![1, 1, 1], vec![1, 8, 1], vec![1, 1, 1]], 0),
            (
                vec![
                    vec![9, 9, 9, 9],
                    vec![9, 0, 0, 9],
                    vec![9, 0, 0, 9],
                    vec![9, 9, 9, 9],
                ],
                36,
            ),
            (
                vec![
                    vec![9, 9, 9, 9, 9],
                    vec![9, 0, 1, 2, 9],
                    vec![9, 7, 8, 3, 9],
                    vec![9, 6, 5, 4, 9],
                    vec![9, 9, 9, 9, 9],
                ],
                45,
            ),
            (
                vec![
                    vec![8, 8, 8, 8, 6, 6, 6, 6],
                    vec![8, 0, 0, 8, 6, 0, 0, 6],
                    vec![8, 0, 0, 8, 6, 0, 0, 6],
                    vec![8, 8, 8, 8, 6, 6, 6, 0],
                ],
                56,
            ),
            (
                vec![
                    vec![0, 10, 0, 20, 0],
                    vec![20, 0, 30, 0, 40],
                    vec![0, 40, 0, 50, 0],
                    vec![50, 0, 60, 0, 70],
                    vec![0, 60, 0, 70, 0],
                ],
                150,
            ),
            (
                vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 0, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                ],
                0,
            ),
            (
                vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 2, 2, 2, 3],
                    vec![3, 3, 3, 2, 3],
                    vec![3, 1, 1, 1, 3],
                    vec![3, 1, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                ],
                0,
            ),
            (
                vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 0, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 1, 3],
                ],
                11,
            ),
            (
                vec![
                    vec![3, 3, 3, 1, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 0, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 3, 3],
                ],
                11,
            ),
        ];

        for (map, expected) in tests.iter() {
            pretty_test(map, *expected);
        }
    }

    #[test]
    fn negative_heights_tests() {
        let tests = [
            (vec![vec![-1]], 0),
            (
                vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 0, 3],
                    vec![3, 0, -2, 0, 3],
                    vec![3, 0, 3, 3, 3],
                    vec![3, 0, 0, 0, 3],
                    vec![3, 3, 3, 1, -3],
                ],
                13,
            ),
            (
                vec![
                    vec![8192, 8192, 8192, 8192],
                    vec![8192, -8192, -8192, 8192],
                    vec![8192, -8192, -8192, 8192],
                    vec![8192, 8192, 8192, 8192],
                ],
                65536,
            ),
        ];

        for (map, expected) in tests.iter() {
            pretty_test(map, *expected);
        }
    }

    #[test]
    fn large_map_test() {
        // 50x50 map without leaks; 100 around the border, 0 inside
        let mut map = vec![vec![100; 50]; 50];
        for y in 1..49 {
            for x in 1..49 {
                map[y][x] = 0;
            }
        }
        // volume = 100 * (48 * 48)
        pretty_test(&map, 230_400);
    }
}
