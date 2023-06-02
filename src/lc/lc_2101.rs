// https://leetcode.com/problems/detonate-the-maximum-bombs/
// 2101. Detonate the Maximum Bombs
pub struct Solution;
impl Solution {
    fn denote_bomb(
        bombs: &Vec<Vec<i32>>,
        denote: &mut Vec<std::collections::HashSet<usize>>,
        direct_denote: &mut Vec<Option<std::collections::HashSet<usize>>>,
        i: usize,
    ) {
        let mut visited = vec![false; bombs.len()];
        let mut q = vec![i];
        while let Some(ii) = q.pop() {
            if visited[ii] {
                continue;
            }
            visited[ii] = true;
            if !denote[ii].is_empty() {
                for j in denote[ii].clone() {
                    denote[i].insert(j);
                    visited[j] = true;
                }
                continue;
            }
            denote[i].insert(ii);
            if let None = direct_denote[ii] {
                let mut d = std::collections::HashSet::<usize>::new();
                for j in 0..bombs.len() {
                    if j == ii {
                        continue;
                    }
                    let dist = (((bombs[ii][0] - bombs[j][0]) as f64).powf(2.0)
                        + ((bombs[ii][1] - bombs[j][1]) as f64).powf(2.0))
                    .sqrt();
                    if dist <= bombs[ii][2] as f64 {
                        d.insert(j);
                    }
                }
                direct_denote[ii] = Some(d);
            }
            if let Some(d) = direct_denote[ii].clone() {
                for j in d {
                    if visited[j] {
                        continue;
                    }
                    q.push(j);
                }
            }
        }
    }
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut denote: Vec<std::collections::HashSet<usize>> =
            vec![std::collections::HashSet::new(); bombs.len()];
        let mut direct_denote: Vec<Option<std::collections::HashSet<usize>>> =
            vec![None; bombs.len()];
        for i in 0..bombs.len() {
            Self::denote_bomb(&bombs, &mut denote, &mut direct_denote, i);
        }
        let mut max = 0;
        for d in denote {
            max = std::cmp::max(max, d.len());
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_detonation() {
        assert_eq!(
            Solution::maximum_detonation(vec_vec![
                [5126, 860, 9831],
                [4128, 6538, 6828],
                [3707, 229, 1412],
                [1720, 8410, 595],
                [1014, 6912, 6100],
                [733, 1810, 4663],
                [2312, 3440, 4697],
                [8121, 2496, 6218],
                [261, 2581, 4824],
                [5571, 8109, 2967],
                [9094, 1510, 4295],
                [6514, 9088, 5690],
                [5444, 8548, 3384],
                [890, 3741, 7561],
                [5144, 8160, 5584],
                [4263, 1102, 1387],
                [9148, 8750, 1720],
                [3089, 406, 5741],
                [9072, 9219, 858],
                [8150, 5001, 8670],
                [6372, 1877, 3064],
                [6024, 9216, 6458],
                [6929, 4628, 7317],
                [413, 8248, 4162],
                [7819, 3037, 2627],
                [7354, 3436, 9656],
                [2909, 9167, 735],
                [1591, 7685, 2299],
                [7972, 5199, 5612],
                [7199, 268, 2894],
                [4944, 984, 543],
                [366, 3435, 6206],
                [5190, 4720, 1021],
                [3647, 7428, 3462],
                [4570, 7546, 8957],
                [9341, 3901, 6531],
                [8247, 7930, 893],
                [1955, 6554, 391],
                [9668, 683, 570],
                [2782, 9952, 393],
                [9414, 8035, 1514],
                [4761, 1789, 5212],
                [2871, 3961, 1479],
                [3658, 7995, 382],
                [1856, 8998, 5943],
                [6781, 6164, 2378],
                [5374, 2436, 5689],
                [1034, 3997, 891],
                [4768, 8011, 3270],
                [4087, 2173, 3897],
                [2110, 4010, 4901],
                [9315, 9202, 946],
                [1647, 1007, 4744],
                [2553, 6157, 1556],
                [6692, 4579, 6936],
                [4097, 6602, 6778],
                [9260, 4730, 9031],
                [3725, 8971, 1800],
                [3610, 8699, 2513],
                [58, 3395, 1349],
                [2606, 9467, 1505],
                [4910, 7593, 4949],
                [7851, 4371, 3562],
                [7891, 4794, 5477],
                [5828, 6879, 738],
                [8769, 5225, 8524],
                [1256, 3866, 6873],
                [5811, 567, 7497],
                [1555, 4445, 4065],
                [1665, 5432, 8917],
                [4613, 8591, 220],
                [9466, 7872, 3327],
                [1332, 7973, 2225],
                [4715, 8696, 1756],
                [2605, 7672, 5947],
                [1302, 9224, 4871],
                [5093, 6084, 932],
                [3797, 2795, 4490],
                [4428, 7148, 4475],
                [3295, 6271, 947],
                [9941, 8043, 3297],
                [4438, 7655, 2876],
                [1454, 1680, 1479],
                [448, 2648, 4043],
                [1520, 6451, 1571],
                [8046, 5409, 7047],
                [5770, 3698, 4924],
                [205, 1482, 5248],
                [7591, 1886, 5044],
                [6077, 8041, 2030],
                [2772, 6899, 9566],
                [1613, 2447, 8322],
                [2609, 5757, 557],
                [8571, 5565, 5404],
                [7277, 5146, 2851],
                [2061, 6765, 2237],
                [6670, 9526, 3239],
                [507, 2049, 622],
                [620, 647, 6590],
                [5231, 9516, 5229]
            ]),
            100
        );
        assert_eq!(
            Solution::maximum_detonation(vec_vec![
                [634, 440, 278],
                [748, 509, 396],
                [995, 881, 251],
                [704, 214, 341],
                [832, 972, 238],
                [987, 384, 156],
                [378, 988, 402],
                [743, 557, 252],
                [814, 868, 196],
                [131, 922, 199],
                [13, 398, 444],
                [464, 607, 241],
                [426, 128, 81]
            ]),
            12
        );
        assert_eq!(
            Solution::maximum_detonation(vec_vec![[2, 1, 3], [6, 1, 4]]),
            2
        );
        assert_eq!(
            Solution::maximum_detonation(vec_vec![[1, 1, 5], [10, 10, 5]]),
            1
        );
        assert_eq!(
            Solution::maximum_detonation(vec_vec![
                [1, 2, 3],
                [2, 3, 1],
                [3, 4, 2],
                [4, 5, 3],
                [5, 6, 4]
            ]),
            5
        );
    }
}
