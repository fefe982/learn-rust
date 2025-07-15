// https://leetcode.com/problems/select-cells-in-grid-with-maximum-score/
// 3276. Select Cells in Grid with Maximum Score
pub struct Solution;
impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let nrow = grid.len() as i32;
        let mut numset = std::collections::BTreeSet::<(i32, usize)>::new();
        for (i, row) in grid.into_iter().enumerate() {
            for n in row {
                numset.insert((n, i));
            }
        }
        let mut used = vec![false; nrow as usize];
        let mut nums = vec![];
        let mut last_val = 0;
        let mut last_row = nrow as usize;
        for (n, i) in numset.into_iter().rev() {
            if n != last_val && last_row != nrow as usize {
                used[last_row as usize] = true;
            }
            if !used[i] {
                nums.push((n, i));
            }
            if n != last_val {
                last_row = i;
            } else {
                last_row = nrow as usize;
            }
            last_val = n;
        }
        println!("{nums:?}");
        let mut max = 0;
        let mut next = 0;
        let mut q = vec![];
        let mut cnt = 0;
        let mut sum = 0;
        let mut set = vec![false; 101];
        used.fill(false);
        loop {
            let mut backtrack = true;
            for j in next..nums.len() {
                let (n, i) = nums[j];
                if used[i] || set[n as usize] {
                    continue;
                }
                q.push((n, i, j));
                println!("{q:?}");
                used[i] = true;
                set[n as usize] = true;
                next = j + 1;
                sum += n;
                cnt += 1;
                if cnt < nrow && next < nums.len() && sum + nums[next].0 * (nrow - cnt) > max {
                    println!("{nums:?}, {next}, {sum}, {cnt}, {nrow}, {max}");
                    backtrack = false;
                }
                break;
            }
            if backtrack {
                max = max.max(sum);
                if q.is_empty() {
                    break;
                }
                let (n, i, j) = q.pop().unwrap();
                used[i] = false;
                set[n as usize] = false;
                sum -= n;
                cnt -= 1;
                next = j + 1;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_score() {
        assert_eq!(
            Solution::max_score(vec_vec![
                [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                [11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
                [21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
                [31, 32, 33, 34, 35, 36, 37, 38, 39, 40],
                [41, 42, 43, 44, 45, 46, 47, 48, 49, 50],
                [51, 52, 53, 54, 55, 56, 57, 58, 59, 60],
                [61, 62, 63, 64, 65, 66, 67, 68, 69, 70],
                [71, 72, 73, 74, 75, 76, 77, 78, 79, 80],
                [81, 82, 83, 84, 85, 86, 87, 88, 89, 90],
                [91, 92, 93, 94, 95, 96, 97, 98, 99, 100]
            ]),
            550
        );
        assert_eq!(Solution::max_score(vec_vec![[5], [7], [19], [5]]), 31);
        assert_eq!(Solution::max_score(vec_vec![[1, 2, 3], [4, 3, 2], [1, 1, 1]]), 8);
        assert_eq!(Solution::max_score(vec_vec![[8, 7, 6], [8, 3, 2]]), 15);
    }
}
