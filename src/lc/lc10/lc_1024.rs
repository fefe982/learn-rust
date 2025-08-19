// https://leetcode.com/problems/video-stitching/description/
// 1024. Video Stitching
pub struct Solution;
impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let mut clips = clips;
        clips.sort_unstable();
        let mut ans = 0;
        let mut cur = 0;
        let mut next = 0;
        for clip in clips {
            if clip[0] > cur {
                if clip[0] <= next {
                    cur = next;
                    ans += 1;
                } else {
                    return -1;
                }
            }
            if clip[0] <= cur && clip[1] >= next {
                next = clip[1];
                if next >= time {
                    return ans + 1;
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn video_stitching() {
        assert_eq!(
            Solution::video_stitching(vec_vec![[0, 2], [4, 6], [8, 10], [1, 9], [1, 5], [5, 9]], 10),
            3
        );
        assert_eq!(Solution::video_stitching(vec_vec![[0, 1], [1, 2]], 5), -1);
        assert_eq!(
            Solution::video_stitching(
                vec_vec![
                    [0, 1],
                    [6, 8],
                    [0, 2],
                    [5, 6],
                    [0, 4],
                    [0, 3],
                    [6, 7],
                    [1, 3],
                    [4, 7],
                    [1, 4],
                    [2, 5],
                    [2, 6],
                    [3, 4],
                    [4, 5],
                    [5, 7],
                    [6, 9]
                ],
                9
            ),
            3
        );
    }
}
