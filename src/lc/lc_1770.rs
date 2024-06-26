// https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/
// 1770. Maximum Score from Performing Multiplication Operations
pub struct Solution;
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut dp = vec![vec![i32::MIN; m + 1]; m + 1];
        for i in 0..=m {
            dp[i][m - i] = 0;
        }
        for i in (0..m).rev() {
            let mul = multipliers[i];
            for j in 0..=i {
                dp[j][i - j] = dp[j][i - j]
                    .max(dp[j][i - j + 1] + mul * nums[n - i + j - 1])
                    .max(dp[j + 1][i - j] + mul * nums[j]);
            }
        }
        dp[0][0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_score() {
        assert_eq!(
            Solution::maximum_score(
                vec![
                    -854, -182, -247, -358, 418, -575, -610, -469, 436, 800, 763, 844, -991, 879, -46, -214, 16, -560,
                    594, -589, 700, 889, -937, -564, 518, -983, 88, 511, 730, -151, 25, 880, -222, 715, -398, -957,
                    -81, 2, -394, -113, 694, 516, 661, -691, -742, 719, -641, 788, -862, 577, -445, -956, -546, 720,
                    915, 574, -75, -373, -847, -471, 2, -763, -940, -115, -579, -443, 760, 384, 693, -676, 955, 877,
                    -31, 445, -593, 571, -173, -550, 636, 306, -402, -143, -155, -367, -224, -151, -333, -330, 93,
                    -692, -875, 149, -525, -32, -561, 202, 767, -723, -827, 870, 482, -661, -156, 578, -118, -872,
                    -128, 307, 267, -239, -551, -660, 130, 185, 577, 756, -192, -910, 424, 820, -545, -705, -700, -496,
                    793, 996, -488, -279, -706, 237, 398, 139, -46, 97, -145, 900, 15, 135, 98, -947, -595, -30, 384,
                    -763, 516, 505, -283, 778, -824, 328, 420, 500, -597, 664, 243, -336, -374, -505, 865, -124, -486,
                    681, -547, 149, 882, 785, -743, -38, -42, -395, -146, 902, -678, 297, 994, 805, -564, 854, -345,
                    -798, 656, 984, -643, -88, -224, -331, 51, 426, 26, -8, -482, -692, -997, 157, 820, -276, -770,
                    -221, 645, 931, -325, -939, -231, 332, -735, -868, -535, 26, -52, 93, -423, -768, -861, -587, -721,
                    -723, -899, 699, -873, 673, -442, 426, -548, 347, -85, 721, -223, 678, 738, 985, 322, 745, 654,
                    -395, -900, -498, 737, 711, -218, 851, 257, -908, 205, 519, 336, -97, -999, -213, -984, -46, 855,
                    626, -184, 270, 757, 908, 349, -273, 442, 449, 645, 804, 73, -376, -259, 786, 486, -823, -171, 424,
                    871, -975, 987, 279, -949, -811, 936, 427, 733, -402, 782, -839, 155, 431, -535, -506, 713, -233,
                    -228, 104, 309, -998, 929, -510, 219, -816, 946, 824, -349, -431, -903, 328, 424, -477, 575, -457,
                    625, 953, 665, -167, 700, -398, 256, 108, 745, 849, 617, -356, 387, 553, 946, -437, -868, -389,
                    -145, 866, 301, -988, 69, -350, -721, -837, -908, 987, -744, 932, -779, -581, 788, 64, 735, 327,
                    752, -890, 402, -207, -650, 274, -947, -61, -511, -839, -230, 760, -34, 635, 617, -569, 542, 185,
                    145, -682, -434, 851, -331, 328, -464, 78, -908, 536, 302, 275, -972, 353, -741, 465, 393, -739,
                    955, 456, 894, -340, -878, 966, 267, -936, -448, -51, -874, 968, 121, -505, -157, 713, 106, -531,
                    -375, -692, -179, -982, -341, -671, 262, 667, -116, 973, -630, 653, 663, -779, 161, -836, -580,
                    -17, -848, -933, 751, 809, -792, 538, -470, 752, 701, -643, 738, -593, 420, 301, 358, 822, 194,
                    499, -621, -924, -172, 892, -839, 833, -844, 825, -728, -397, 132, -947, -329, 483, -744, -95,
                    -577, -30, 252, -964, -22, 588, -47, 123, 630, -472, -382, 123, -936, 559, -942, 121, -481, -203,
                    -935, -78, 158, -831, 146, 153, -671, -395, 579, 141, 269, -949, -187, -156, -334, 606, 292, 58,
                    -560, 641, 923, -451, 597, -698, 855, -735, 425, 332, 507, 956, -78, -274, -820, -26, 127, -685,
                    -346, 655, -270, -282, -815, 200, 996, -554, -914, 455, -353, 662, 715, -279, 833, 801, 611, -924,
                    879, 990, 540, -942, 845, 1000, -600, 634, -634, -11, -927, 623, -789, 489, -534, -806, 396, 510,
                    303, -256, 392, -730, -328, 734, 878, 799, -233, -8, -965, 206, -449, 424, 737, -480, -934, 491,
                    250, 208, 114, 151, 870, -237, 301, 975, -519, 508, -843, 939, -42, -993, 662, 249, -250, -779,
                    641, -587, 419, 169, 450, -989, -843, -314, -229, -43, 679, 350, -502, -46, -430, -99, -595, -533,
                    -34, 938, -304, -647, 475, -925, -649, -539, 509, 421, -77, 874, -708, 997, 27, -22, -212, -36,
                    -402, -135, 83, -367, 817, 445, -461, 518, -857, -435, 415, -216, -3, 443, -194, 712, 457, -824,
                    -280, -664, -61, 433, 279, 265, 960, -822, 704, -75, 558, -16, -371, -398, 73, -358, -472, 829,
                    108, -45, -88, 672, 247, 744, -685, 78, 420, 618, 584, 241, 187, 883, 628, -10, 925, 120, -93,
                    -564, -790, 876, 847, 41, -67, -326, -207, 111, 787, -517, 564, 525, -644, 265, -349, -941, -375,
                    585, -644, -797, 361, 580, 164, 289, 825, -52, -833, 757, 300, 803, 7, -361, 555, 66, -960, -303,
                    577, -587, -129, -73, 351, -142, 342, -756, 938, 33, 225, -747, -978, 756, 661, -873, 620, -946,
                    512, -314, -42, 552, -726, -842, -789, -408, -642, -440, 483, -582, 304, -711, -138, 249, 877,
                    -248, -11, 797, 444, -923, 612, 352, -281, 456, -104, 894, 918, 956, -725, 153, 649, -773, 652,
                    -297, 393, -384, -642, -147, -650, -998, -378, 695, 351, -981, -701, -171, 490, 623, 362, 499,
                    -947, -74, 682, -963, 658, 757, 362, 795, -458, 533, 137, -98, -156, 50, 585, 657, -10, -424, 937,
                    -896, 606, 910, 67, 308, -234, 435, 214, -666, 102, 490, 77, -241, 23, -999, 501, -416, -373, 574,
                    438, -541, -623, 781, -959, -751, -511, -740, -831, -540, -132, -875, 495, 915, 592, -503, -216,
                    492, 158, -833, 186, -645, -41, 22, 544, -861, -540, -372, 251, 768, -389, 559, 60, -838, -394,
                    131, 832, -536, -559, 450, -295, -939, -805, -517, 887, 18, -559, -522, 699, -733, 670, 528, -172,
                    108, -461, 726, 81, -846, -961, 697, -699, 961, 441, 558, -390, -397, -435, 194, 738, -883, -515,
                    -78, -687, 725, 133, -654, 942, -887, 839, 332, 141, -751, 389, -512, -114, -750, -519, -248, -47,
                    196, 916, 118, -734, -182, -478, -836, 482, 187, -720, 649, -251, -661, 991, -714
                ],
                vec![
                    894, -252, -68, 851, 874, -539, 804, 907, 7, -904, 544, 37, 688, -133, 385, -823, 288, -350, -15,
                    820, -408, 241, -754, -879, -96, 997, -784, -791, -65, 142, -271, -521, 409, 760, 927, -29, -30,
                    -803, -454, 520, 789, -557, 283, -218, 226, 746, -111, -385, 310, -531, -79, 40, 357, 118, 854,
                    -369, -347, -109, 771, 795, -615, -153, 356, -292, -56, -105, 961, -212, 32, 402, -322, -933, 268,
                    -3, 913, 917, 641, 658, 878, 908, 518, 228, 723, -505, -815, 701, -339, -27, -512, 810, 43, -524,
                    818, -698, 935, -357, -252, 359, 695, 221, 94, 552, -371, -993, 640, 761, -101, -130, 112, 85,
                    -204, 646, -482, 972, -301, 498, 22, -45, -911, 525, 229, 769, 202, 122, 282, 469, -333, -487,
                    -725, -962, 807, 613, 427, 21, 41, -209, 698, 749, 16, 253, 600, 517, 200, 873, -800, 187, 253,
                    990, 647, -191, -365, -181, -191, -820, -583, 958, 823, -618, 28, 165, 567, -368
                ]
            ),
            39354388
        );
        assert_eq!(Solution::maximum_score(vec![1, 2, 3], vec![3, 2, 1]), 14);
        assert_eq!(
            Solution::maximum_score(vec![-5, -3, -3, -2, 7, 1], vec![-10, -5, 3, 4, 6]),
            102
        );
    }
}
