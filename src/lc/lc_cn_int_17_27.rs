// https://leetcode.cn/problems/sparse-similarity-lcci/
// 面试题 17.26. 稀疏相似度
pub struct Solution;
impl Solution {
    pub fn compute_similarities(docs: Vec<Vec<i32>>) -> Vec<String> {
        let mut rmap = std::collections::HashMap::<i32, Vec<usize>>::new();
        let mut cnt = std::collections::HashMap::<(usize, usize), i32>::new();
        for (i, doc) in docs.iter().enumerate() {
            for &val in doc {
                if let Some(v) = rmap.get_mut(&val) {
                    for &j in v.iter() {
                        *cnt.entry((j, i)).or_default() += 1;
                    }
                    v.push(i);
                } else {
                    rmap.insert(val, vec![i]);
                }
            }
        }
        let mut res = vec![];
        for ((i, j), v) in cnt {
            let v0 = v as i64 * 10000;
            let v1 = (docs[i].len() + docs[j].len() - v as usize) as i64;
            let mut vv = v0 / v1;
            if v0 % v1 * 2 >= v1 {
                vv += 1;
            }
            res.push(format!("{},{}: 0.{:04}", i, j, vv));
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_compute_similarities() {
        assert_sort_eq!(
            Solution::compute_similarities(vec_vec![
                [
                    9988, 69510, 32781, 35092, 86036, 93591, 73124, 58664, 47402, 27318, 29113, 47551, 80195, 68805,
                    64070, 82634, 81098, 20305, 53079, 40795, 71389, 70373, 20460, 74736, 90609
                ],
                [
                    47622, 89873, 14489, 62747, 24475, 159, 29856, 94503, 88880, 2098, 73268, 47672, 72130, 1605,
                    68303, 57298, 59094, 84185, 49754, 15582, 33376, 84325, 96617, 47722, 44778, 75500, 72047, 51184,
                    34805, 27128
                ],
                [
                    86532, 30984, 60304, 53009, 33811, 35099, 90396, 19365, 4262, 89128, 14763, 4012, 23723, 57149,
                    53699, 20938, 87884, 37718, 74073, 89307, 88668, 92515, 9445, 67563, 55534, 15865
                ],
                [
                    88578, 48263, 25607, 2824, 2314, 10122, 76551, 62475, 30352, 58386, 56212, 47767, 27672, 80283,
                    51996, 50080, 18172, 63914, 28331, 29741, 1071, 87868, 40893, 29885, 31037, 29637, 72190, 45896,
                    14409, 68688, 77649, 9945, 78180, 93928, 1514, 17902, 59890, 10745, 7420, 79870
                ],
                [
                    25484, 2704, 2321, 57237, 1562, 4378, 8483, 32293, 61481, 15920, 76720, 34101, 9527, 2872, 21577,
                    61002, 7888, 24272, 95449, 22746, 62302, 96486, 45930, 26102, 1401
                ],
                [
                    38241, 75010, 21539, 31076, 81865, 18570, 3443, 5715, 15540, 18742, 10103, 68566, 80857, 48159,
                    9375
                ],
                [7269, 7946, 250, 13131, 4333, 56661, 97784, 9689, 1722, 26557, 78175],
                [
                    76041, 11914, 58124, 6425, 85916, 3100, 55586, 30378, 14128, 21168, 72368, 9907, 28726, 38464,
                    9025, 49741, 11982, 69069, 84834, 67044, 6758, 30311, 78953, 66285, 99054, 65651, 39929
                ],
                [
                    43018, 28044, 58772, 79991, 72093, 71073, 15139, 2860, 35892, 1722, 3447, 54603, 24524, 41547,
                    43606, 41945, 35929, 90335, 18536, 41580, 39543, 9464, 5497, 7931, 20092, 97534
                ],
                [
                    47493, 80518, 75143, 54536, 66318, 86414, 40211, 95251, 90006, 6425, 5629, 84644, 58411, 17835,
                    73388, 61998, 79659, 31925, 58295, 44093, 5193, 66382, 209, 93907, 76137, 41071, 12785, 98674,
                    44403, 95223, 52093
                ],
                [
                    35586, 67715, 69023, 64674, 98853, 8233, 60716, 15543, 23482, 71492, 14541, 3919, 1616, 7120,
                    56148, 63710, 68708, 56944, 46588
                ],
                [
                    14977, 68360, 33161, 4363, 36370, 70420, 86935, 15383, 26140, 59177, 5678, 91183, 90931, 949,
                    39478, 41398, 9405, 91582, 81087, 59072, 43585, 63303, 44628, 5593, 27099, 8286, 45920, 30565,
                    92392, 21225, 64241, 39153, 7929, 64890, 52350
                ],
                [
                    40320, 2442, 63242, 67724, 5038, 90286, 7473, 75702, 9400, 12729, 61248, 76100, 43848, 78544, 4828,
                    84707, 60132, 71780, 34922, 98425
                ],
                [
                    95106, 4741, 60304, 9623, 1436, 81188, 74160, 45873, 1968, 21556, 7733, 50230, 20408, 37700, 45638,
                    16071, 73935, 78799, 95055, 340, 42454, 34393, 38493, 25694, 23146, 93551, 66804
                ],
                [
                    58765, 60433, 56467, 76820, 12566, 58394, 34714, 89763, 5038, 58929, 59698, 38067, 47420, 14785,
                    28354, 53442, 58699, 71887, 99801, 93402, 83932, 5853, 45278, 97759, 76000, 1504, 18016, 82403,
                    47199, 50277, 68709, 19699, 10358, 47734, 31352, 66684
                ],
                [5894, 9897, 43757, 93935, 46065, 57080],
                [
                    51591, 13321, 52366, 77585, 49043, 19476, 17946, 9243, 5916, 17952, 54439, 3496, 2229, 93111,
                    17215, 81730, 23496, 9939, 44632, 6627, 34148, 88423, 3306
                ],
                [4993, 65639, 82714, 33352, 76686, 21519, 55388, 2229, 81269, 7350, 71509, 11258, 9243, 81500, 74494],
                [
                    90112, 83721, 67851, 84877, 3985, 60818, 85529, 77985, 8487, 80682, 42539, 18223, 76995, 91080,
                    83273, 458, 52814, 61135, 47324, 75101, 863, 77028, 78310, 90860, 50672, 25840, 13179, 72189
                ],
                [6656, 7936, 41058, 43618, 9508, 427, 25087, 24464, 38839, 21438, 26391, 66237, 36318, 8927],
                [16288, 75940, 4741, 35268, 7399, 1448, 3588, 25037, 8598],
                [
                    2176, 25219, 23687, 63759, 71824, 62352, 33427, 9495, 26391, 675, 40229, 14758, 86828, 87726,
                    55470, 49456, 2737, 73530, 73402, 44994, 40260, 61252, 25412, 5191, 63307, 84947, 21335, 16729,
                    73310, 68450, 92006, 103, 9835, 4845, 37357, 59760, 50673, 87921, 40562, 66804, 57467, 61052,
                    74110
                ],
                [
                    8960, 82695, 3465, 86283, 59019, 96282, 32795, 94875, 44199, 30390, 5949, 48191, 70593, 31300,
                    94406, 70983, 4680, 37193, 95952, 87003, 6619, 39005, 28897, 49892, 48870, 54510, 29811, 63987,
                    49147
                ],
                [
                    48001, 71169, 23171, 8456, 54668, 57231, 66578, 34843, 90793, 71596, 59311, 39215, 27439, 46257,
                    28083, 79284, 27445, 79672, 38841, 64826, 25531, 43579, 75837, 83647, 67, 2756, 56773, 7111, 41544,
                    85623, 80074, 66505, 32631, 86609, 51286, 18518, 68954, 28387, 93412, 23781, 66021, 8935, 13544,
                    71401, 34155, 54125, 85238, 15990, 20088, 6910
                ],
                [
                    50305, 63747, 51972, 71815, 92298, 65559, 23961, 24859, 18077, 65441, 12453, 91818, 51001, 24892,
                    24126, 78527, 40000, 27333, 12357, 41941, 82138, 17114, 87388, 38243, 11875, 57451, 23029, 18038,
                    36086
                ],
                [
                    22402, 90243, 64260, 7941, 6150, 81159, 5128, 58504, 89991, 16397, 3344, 88982, 35222, 84249, 7067,
                    800, 60330, 83756, 57014, 29120, 29007, 72143, 22104, 91737, 94561, 64870, 53094, 28906, 49130,
                    45674, 14571, 77045, 49654, 2936
                ],
                [35904, 53186, 24803, 98886, 18702, 16148, 27417, 62522, 41725],
                [
                    54400, 84100, 47113, 79756, 38928, 51356, 6943, 7330, 64421, 14762, 4653, 55857, 3899, 63037,
                    95425, 20294, 53833, 92497, 51799, 70109, 33247, 58081, 25954, 3685, 74342, 72941, 94069, 20350,
                    59135
                ],
                [9284, 51077, 20485, 33064, 94027, 18508, 3985, 20307, 33588, 2069, 82263, 68122, 79292, 41246],
                [
                    66435, 36613, 5514, 94478, 35857, 37393, 63251, 61588, 73495, 26871, 69543, 43949, 72881, 41397,
                    87097, 91199, 25037, 31200, 16109, 92530, 88690, 32887, 5881
                ],
                [56576, 93890, 64972, 20079, 3537, 3698, 29300, 8094],
                [
                    24323, 34322, 1298, 61973, 55078, 13351, 20391, 50348, 42413, 75956, 90036, 51768, 39994, 29119,
                    79040, 42691, 85316, 11337, 94414, 55250, 93014, 94678, 863, 58851, 4969, 17257, 37486, 89339
                ],
                [864, 31292, 85277],
                [
                    48266, 52112, 2192, 68893, 7069, 34589, 72609, 12069, 57132, 61230, 78769, 86582, 16191, 15171,
                    41163, 3659, 27618, 29670, 25958, 5488, 35441, 37113
                ],
                [
                    25474, 97286, 13705, 34705, 61458, 34584, 98201, 19875, 42418, 87988, 43705, 14785, 68291, 80339,
                    85718, 59245, 91643, 53244, 31357
                ],
                [
                    71426, 40835, 92931, 14725, 49157, 86411, 68627, 88213, 85912, 15132, 22813, 17954, 86307, 79908,
                    6437, 69925, 67495, 13099, 5166, 86446, 38704, 35252, 21942, 958, 13374, 69571, 40011, 73294,
                    60881, 75866, 4443, 48864, 21731, 6117, 71142, 86249, 80618, 78443, 97648, 26740, 4726, 14839,
                    9596
                ],
                [
                    7811, 66053, 7814, 80391, 2312, 11144, 84486, 71314, 62237, 37920, 89505, 95399, 54314, 37419,
                    6446, 26800, 24633, 14267, 2369, 92487, 32967, 58057, 10316, 11468, 75730, 99411, 80724, 18389,
                    83541, 88279, 80082, 96982, 21077, 11998, 17247, 88159, 48611, 12131, 42470, 9835, 8044, 56557,
                    63600, 85873, 93680, 89715, 36344, 53242, 26876, 85629
                ],
                [
                    68354, 91525, 83974, 42375, 45326, 77599, 62247, 5548, 43697, 62388, 7733, 11316, 70452, 18123,
                    65356, 209, 6748, 84960, 64867, 27113, 95213, 7926, 72697
                ],
                [
                    16003, 71558, 44942, 73106, 6547, 29971, 42773, 89365, 24345, 93722, 95262, 75940, 4519, 23848,
                    63530, 42538, 4528, 81077, 20666, 80702, 60991, 84928, 89150, 59330, 75591, 53449, 99529, 6987,
                    47309, 55124, 29397, 82006, 9047, 99419, 18396, 41694, 28128, 84832, 42083, 31332, 1254, 76134,
                    54128, 66417, 63602, 28920
                ],
                [42561, 2979, 73271, 14022, 1451, 1231, 72049, 48030, 18423, 83166, 6495],
                [60192, 6593, 52225, 74880, 67524, 36098, 166, 34886, 63915, 69266, 75036, 70198, 83929, 8954, 64604],
                [73090, 92389, 5004, 6224, 85744, 37968, 6515, 39060, 16436, 95446, 48214, 61016, 58748],
                [2080, 94402, 63458, 71013, 82183, 47816, 64456, 5548, 61804, 12863, 18932, 51060, 41372, 8415],
                [
                    20490, 90635, 96267, 72853, 3991, 58007, 16926, 20511, 76961, 26275, 37929, 6201, 88122, 82618,
                    41915, 46653, 72633, 8258, 95049, 458, 7501, 86093, 41433, 12000, 38631, 11887, 71664, 2805, 15093,
                    12790, 1784, 98171
                ],
                [
                    30467, 19346, 41491, 71197, 45345, 293, 95273, 10929, 55602, 2872, 59070, 7230, 6981, 52174, 22491,
                    78435, 38499, 42605, 60529, 5622, 17655, 1784, 8566, 19190
                ],
                [
                    5124, 31751, 36615, 12935, 67608, 27161, 68383, 64928, 87459, 68773, 33065, 80812, 7345, 84917,
                    22329, 6841, 959, 66500, 4805, 86983, 87623, 2377, 23249, 70482, 18003, 23637, 27353, 66267, 73435,
                    20319, 90976, 482, 12130, 34279, 85736, 41961, 84973, 47218, 21363, 8566, 62583, 7033, 7547, 43903
                ],
                [
                    43169, 29474, 15139, 24548, 24644, 19873, 4935, 80225, 18121, 86733, 62620, 53876, 4921, 91899,
                    8444, 5724
                ],
                [
                    68868, 1797, 64644, 54149, 35214, 46611, 79894, 32536, 21785, 12185, 71201, 18723, 20261, 30631,
                    427, 47736, 24109, 60078, 36269, 5173, 69943, 72503, 2107, 3656, 46286, 29779, 73592, 22136
                ],
                [69920, 7153, 3443, 86037, 2808, 29116, 86269],
                [
                    3336, 273, 27668, 87316, 8981, 18328, 33568, 5283, 93732, 18213, 37412, 40235, 5164, 66350, 58415,
                    61233, 86965, 23097, 13884, 75964, 45887, 27200, 27202, 15948, 36173, 87756, 39247, 12624, 23121,
                    85967, 93012, 55380, 35926, 81751, 84440, 18905, 59739, 66780, 58588, 92515, 25061, 3175, 66920,
                    58605, 67055, 77552, 35057, 27762, 1012, 50942
                ],
                [
                    80515, 1800, 61321, 83720, 60302, 77972, 93973, 13078, 43289, 6041, 81820, 99870, 78368, 78881,
                    53028, 94772, 76982, 26807, 92730, 30780, 89023, 35272, 61131, 2640, 6609, 70230, 5214, 68191,
                    77919, 26083, 53476, 21737, 4845, 10607, 39920, 56946, 6515, 26613, 374, 504, 88186
                ],
                [2808, 346],
                [
                    92737, 67170, 26782, 34467, 30886, 65350, 18250, 25324, 272, 54737, 39248, 52147, 6041, 38490,
                    80219, 99838
                ],
                [
                    900, 6409, 12809, 22797, 69007, 5009, 63250, 97682, 28051, 83094, 73622, 69400, 79639, 47130,
                    21278, 59684, 50221, 69300, 67256, 33338, 80575, 41663, 28869, 69830, 9674, 38736, 75472, 3292,
                    87396, 27887, 45679, 63091, 50548, 49657, 60798
                ],
                [
                    19200, 74113, 22403, 24710, 79240, 94987, 47121, 84626, 82835, 71060, 54676, 19990, 6166, 48156,
                    72991, 56866, 75043, 67748, 95272, 67881, 47406, 30258, 50482, 82483, 34488, 83130, 31290, 5697,
                    19265, 81605, 81478, 38342, 5706, 55115, 42828, 99148, 66894, 90709, 47322, 73184, 64864, 93796,
                    33643, 8176, 8564, 48503, 79615
                ],
                [
                    99106, 35618, 77221, 21576, 45163, 3372, 2155, 69646, 7951, 40783, 7349, 84119, 75323, 9178, 9179,
                    9375
                ],
                [
                    41345, 51081, 51214, 30607, 5525, 69014, 29082, 8094, 86563, 56996, 4262, 48174, 21556, 15414,
                    26296, 53180, 30398, 73027, 87624, 68043, 37710, 40144, 45907, 87508, 97493, 9945, 23388, 1247,
                    53345, 7269, 4969, 96626, 45811, 63220, 62969, 36861
                ],
                [
                    33913, 45443, 26001, 8082, 15505, 31127, 90391, 82585, 67994, 42399, 29088, 95521, 18468, 73898,
                    55982, 48433, 78389, 72249, 83770, 22844, 15432, 8649, 86475, 4686, 49234, 38614, 55513, 21725,
                    27616, 41184, 28896, 70243, 96998, 1126, 12393, 2287, 18549, 74745, 12794, 75643, 21116
                ],
                [
                    43139, 66820, 30472, 28170, 9750, 9246, 5662, 2082, 37676, 1198, 82991, 35762, 86451, 72500, 10810,
                    69060, 73926, 72776, 7888, 58192, 80976, 95965, 93412, 1380, 29413, 51306, 37996, 32884, 38393,
                    37882
                ],
                [
                    52737, 3851, 75021, 1562, 84379, 59551, 59682, 166, 38824, 87469, 5166, 16307, 36150, 7111, 21712,
                    95830, 54892, 75508, 42229, 93302
                ],
                [
                    53633, 3332, 83460, 45323, 61709, 19219, 78358, 93720, 49482, 79691, 94415, 61023, 50144, 87520,
                    50538, 4462, 74606, 8049, 26609, 24819
                ],
                [
                    57351, 37646, 41871, 11292, 4901, 78375, 52531, 12344, 52283, 26059, 91723, 77137, 8787, 49110,
                    72538, 2526, 89567, 15839, 75874, 87140, 95076, 93286, 88166, 22120, 50408, 3835
                ],
                [71265, 45092, 94855, 4935, 32489, 39211, 1171, 65269, 1141, 94973],
                [
                    37888, 38785, 9988, 75013, 48007, 61575, 55820, 85135, 17426, 89750, 78374, 57382, 90800, 30002,
                    43831, 30519, 79673, 87483, 75585, 73025, 45636, 91214, 49744, 64464, 94545, 32464, 98256, 30166,
                    59224, 18142, 42718, 14306, 65122, 5092, 52581, 95591, 24685, 57454, 39790, 5872, 1393, 38390,
                    8700, 63486
                ],
                [
                    73216, 1411, 3332, 41233, 5013, 21278, 13983, 86047, 78882, 85158, 38189, 56750, 58945, 30915,
                    75718, 27722, 8010, 51790, 29158, 26229, 58871
                ],
                [8672, 11202, 5122, 37474, 48293, 8892, 27324, 1002, 65867, 45234, 9939, 22964, 44088, 78460, 38207],
                [25991, 78578, 5782, 82585, 8954, 7067, 69404, 12733],
                [
                    9233, 96276, 93717, 51109, 4519, 37159, 74795, 4653, 10671, 6449, 56753, 89269, 92091, 74044,
                    73027, 11204, 10309, 85830, 79943, 94559, 3685, 88677, 73973, 21242, 19324
                ],
                [
                    90754, 32002, 4100, 41478, 2314, 3213, 24334, 37135, 90001, 59666, 96022, 86551, 94999, 1433,
                    48153, 33945, 10778, 69533, 64670, 43551, 2467, 82853, 67879, 63916, 99117, 940, 17200, 46517,
                    5430, 92350, 23875, 94791, 96456, 84683, 69707, 45903, 92246, 97369, 75613, 63581, 75487, 62560,
                    58083, 37865, 75370, 8813, 17519, 91122, 75124, 23672
                ],
                [
                    11397, 26247, 17416, 93961, 37900, 2323, 19100, 80417, 52773, 67750, 24104, 8490, 11564, 55341,
                    1198, 85169, 35377, 33334, 48701, 52159, 94790, 24648, 10057, 71248, 12755, 13528, 346, 94685,
                    27870, 77663, 42976, 15969, 41441, 8423, 24428, 78199, 62458, 88315, 83708, 77949, 75006
                ],
                [
                    51328, 5508, 74512, 66705, 11537, 18838, 80792, 68652, 55734, 6199, 48696, 46782, 87745, 7501,
                    75470, 14798, 44241, 96984, 8670, 20446, 35682, 74481, 62835, 89333
                ],
                [
                    6656, 36867, 93068, 35341, 65424, 71828, 54039, 73885, 41379, 16291, 77741, 81204, 57916, 68545,
                    38725, 32079, 27472, 8028, 6495, 7519, 73074, 8819, 2422
                ],
                [38243, 34556, 7941, 28231, 1448, 82504, 43112, 89996, 14830, 5715, 54998, 35640, 11835, 4892, 9756],
                [7420],
                [
                    29582, 19087, 1172, 98709, 23061, 68764, 6557, 65314, 63652, 63918, 43066, 1726, 43969, 57815,
                    25187, 29670, 15975, 70001, 34548, 54271
                ],
                [
                    82821, 18320, 89874, 85396, 6166, 58393, 68122, 14106, 16284, 61084, 83742, 54183, 82220, 55470,
                    42034, 9907, 7033, 74170, 1726, 38090, 61390, 40919, 859, 8543, 30688, 41439, 84585, 31850, 65003,
                    9837, 4206, 46063, 35320, 94073, 88957
                ],
                [
                    10625, 27139, 17030, 13585, 81298, 53656, 52510, 6943, 12192, 90788, 63910, 45479, 73272, 51514,
                    92096, 86720, 95426, 61764, 17735, 2761, 22346, 60879, 18898, 64595, 66645, 18902, 67415, 22615,
                    49626, 57435, 4828, 18397, 34910, 8415, 97889, 61539, 43752, 6134, 9079
                ],
                [97088, 99710, 2979, 70130, 12476, 47506, 65364, 85562, 4120, 59482, 94043, 4892, 94814],
                [
                    59143, 85006, 68113, 2322, 84247, 22295, 59932, 11936, 1446, 37930, 16687, 49200, 91316, 87863,
                    18615, 3003, 74301, 6463, 90303, 20289, 95937, 67010, 44870, 14151, 60490, 26317, 6224, 42579, 724,
                    92371, 53975, 50268, 26334, 81759, 95455, 38499, 51427, 59370, 9837, 74992, 30834, 99063, 61308,
                    98173
                ],
                [
                    99200, 38144, 86919, 63884, 84110, 1807, 82325, 35733, 90392, 28570, 26908, 28701, 57372, 2464,
                    54307, 87972, 48038, 38311, 86569, 17973, 9527, 2880, 29889, 16322, 16963, 36932, 81101, 92496,
                    98265, 2395, 65247, 57698, 9445, 12651, 91118, 31092, 55546, 57467, 72062
                ],
                [
                    37248, 27395, 11652, 92547, 74891, 91922, 75801, 42523, 15005, 8490, 37804, 53552, 98097, 46130,
                    34867, 53172, 66360, 87745, 52931, 45764, 67526, 83271, 20809, 19530, 83019, 68179, 28500, 65236,
                    13273, 66534, 55017, 38763, 67440, 74235, 9979, 17789, 4606
                ],
                [
                    81795, 20996, 86149, 64523, 43531, 18443, 79118, 7313, 33555, 60565, 54423, 40221, 67872, 22054,
                    7210, 64299, 44204, 77239, 24890, 74183, 46286, 18263, 65378, 34018, 57060, 7142, 65127, 18792,
                    56683, 45164, 37231, 89842, 85620
                ],
                [
                    89603, 46086, 82313, 64016, 40466, 15635, 66454, 94490, 90523, 34203, 50217, 69802, 31024, 30001,
                    44208, 42804, 98870, 9400, 35899, 76476, 75453, 19393, 3912, 50383, 21201, 63442, 98262, 29272,
                    13786, 8412, 2909, 16988, 73444, 86118, 93543, 55656, 72426, 29803, 55410, 1779, 9720, 22521
                ],
                [
                    17542, 6665, 7305, 21, 49185, 27811, 89133, 30766, 33976, 31800, 87880, 2377, 68810, 39368, 50000,
                    83936, 19560, 53355, 69230
                ],
                [
                    81413, 39947, 87445, 8598, 54681, 61978, 48029, 81569, 89252, 49326, 82226, 84664, 79037, 86216,
                    68813, 50382, 21456, 37201, 1880, 857, 6758, 55272, 48233, 74346, 79336, 69356, 52209, 16246,
                    78969, 17535
                ],
                [
                    89601, 1797, 54152, 79497, 89101, 8974, 75150, 28048, 662, 20375, 31386, 18587, 3870, 79135, 21154,
                    79268, 3496, 51496, 6958, 20017, 84792, 3003, 78396, 15420, 71617, 26855, 57960, 11242, 5229,
                    42989, 20850, 95607, 54013
                ],
                [97863, 19304, 20712, 9674, 47852, 7153, 30359, 32411],
                [
                    54666, 40335, 12182, 4378, 82331, 97438, 5662, 163, 90533, 6446, 33981, 88771, 12488, 27850, 5458,
                    24022, 73053, 7519, 21604, 20852, 374, 377, 48508, 72701
                ],
                [
                    82178, 17795, 43653, 98054, 65159, 35720, 24585, 82826, 57099, 71824, 91415, 10776, 49818, 17178,
                    82846, 53793, 95394, 54691, 23462, 23591, 30505, 97066, 77103, 59442, 90804, 55991, 56, 55609,
                    50240, 83394, 7241, 3018, 56400, 48851, 340, 30035, 50017, 88674, 7399, 32106, 95852, 4080, 74737,
                    53105, 15992, 9977
                ],
                [86081, 28461, 4206, 46607, 9720, 83737, 75998],
                [48605, 163, 34886, 79656, 360, 1451, 9553, 1171, 1172, 94231, 5622, 30390, 76413],
                [
                    52228, 93444, 53004, 19214, 29327, 63632, 27670, 97688, 95897, 13596, 30750, 18339, 13092, 35750,
                    37927, 79021, 99381, 9913, 2234, 95290, 14398, 33225, 3018, 20685, 21326, 7120, 62289, 47826, 3922,
                    29915, 49628, 81374, 58976, 56167, 38506, 746, 18285, 85871, 6128, 74864, 42871, 43257, 18170,
                    51452, 95742
                ],
                [7394, 38440, 2442, 12304, 21752, 5916],
                [
                    74245, 16390, 65288, 44945, 8082, 43673, 60316, 63904, 92193, 67233, 17571, 36645, 52390, 83110,
                    5544, 25257, 72624, 92594, 61365, 2301, 3899, 98115, 66372, 73284, 31942, 82378, 8525, 23246, 3537,
                    6611, 76925, 23383, 88795, 21473, 31971, 79843, 87400, 60523, 50669, 24685, 54895, 86896, 4340,
                    93053, 10110, 81663
                ],
                [
                    31236, 20614, 8585, 65675, 14735, 65551, 62870, 49564, 6430, 89758, 13088, 28195, 78376, 57640,
                    46891, 27055, 23729, 76850, 20277, 30646, 6712, 72383, 34496, 92992, 14789, 47687, 8266, 65867,
                    46795, 9930, 49104, 26972, 3421, 8927, 11361, 92003, 49893, 18537, 17002, 49387, 71537, 26228,
                    9335, 18173
                ],
                [
                    27652, 8456, 42762, 94602, 25872, 80146, 11928, 59165, 79519, 66470, 19369, 85163, 3372, 26928,
                    84529, 24627, 41659, 15299, 49348, 51141, 81989, 99276, 26068, 65240, 74072, 864, 89698, 15595,
                    60781, 55927, 19706, 84091
                ],
                [31329, 30433, 5380, 33798, 56391, 14152, 13352, 31248, 8594, 14707, 84626, 51930],
                [
                    41344, 66498, 41979, 70154, 58847, 1644, 91179, 2703, 31156, 5301, 47862, 5143, 94234, 63646,
                    31772, 89150, 3551
                ],
                [
                    84355, 9354, 57741, 53140, 48405, 1173, 18454, 23704, 83739, 56731, 13092, 15014, 66349, 11055,
                    87732, 2747, 47804, 40128, 56773, 18504, 29385, 585, 33738, 71373, 76878, 78162, 34770, 27865,
                    85082, 65757, 5981, 53471, 73567, 7394, 73314, 27235, 96869, 79203, 20455, 47590, 64874, 93674,
                    72045, 23022, 57072, 84851, 72949, 79093, 16760
                ],
                [
                    20996, 99335, 32904, 33289, 85640, 2056, 55692, 78343, 94222, 3727, 75792, 21755, 12566, 82583,
                    67228, 35486, 79783, 6567, 33964, 6831, 3761, 29238, 70073, 52284, 69438, 97600, 13127, 47691,
                    20430, 1616, 22993, 64851, 50900, 72279, 52953, 5853, 8158, 34400, 44896, 867, 83686, 63465, 90091,
                    83691, 47467, 35695, 73076, 4219, 37244, 86779
                ]
            ]),
            [
                "60,64: 0.0250",
                "18,43: 0.0169",
                "27,67: 0.0385",
                "2,56: 0.0164",
                "35,59: 0.0161",
                "27,76: 0.0149",
                "19,71: 0.0278",
                "36,87: 0.0137",
                "69,80: 0.0130",
                "44,90: 0.0278",
                "2,49: 0.0133",
                "19,94: 0.0175",
                "3,73: 0.0250",
                "27,93: 0.0135",
                "43,44: 0.0182",
                "3,68: 0.0112",
                "44,78: 0.0149",
                "45,83: 0.0161",
                "20,72: 0.0435",
                "12,82: 0.0164",
                "78,85: 0.0132",
                "12,92: 0.0400",
                "70,80: 0.0167",
                "4,87: 0.0208",
                "53,64: 0.0182",
                "28,75: 0.0208",
                "18,31: 0.0182",
                "9,37: 0.0189",
                "18,28: 0.0244",
                "4,79: 0.0159",
                "2,13: 0.0192",
                "20,88: 0.0185",
                "12,76: 0.0172",
                "45,75: 0.0128",
                "20,84: 0.0263",
                "50,52: 0.0179",
                "53,86: 0.0238",
                "0,63: 0.0147",
                "58,69: 0.0143",
                "25,66: 0.0244",
                "92,98: 0.0185",
                "4,58: 0.0185",
                "4,59: 0.0227",
                "20,38: 0.0185",
                "75,89: 0.0244",
                "25,72: 0.0208",
                "4,44: 0.0208",
                "37,42: 0.0278",
                "58,87: 0.0189",
                "10,99: 0.0147",
                "19,21: 0.0179",
                "50,87: 0.0156",
                "42,76: 0.0192",
                "75,78: 0.0128",
                "19,47: 0.0244",
                "3,56: 0.0133",
                "91,98: 0.0108",
                "51,69: 0.0238",
                "10,91: 0.0159",
                "2,79: 0.0156",
                "20,29: 0.0323",
                "43,70: 0.0182",
                "44,45: 0.0149",
                "12,14: 0.0182",
                "65,94: 0.0172",
                "6,56: 0.0217",
                "40,90: 0.0370",
                "48,86: 0.0714",
                "13,21: 0.0145",
                "13,20: 0.0286",
                "56,67: 0.0167",
                "7,84: 0.0179",
                "14,34: 0.0185",
                "7,75: 0.0164",
                "23,95: 0.0123",
                "30,56: 0.0233",
                "40,66: 0.0455",
                "24,72: 0.0233",
                "16,85: 0.0182",
                "57,66: 0.0208",
                "5,48: 0.0476",
                "13,56: 0.0161",
                "23,98: 0.0102",
                "74,90: 0.0313",
                "46,62: 0.0400",
                "16,92: 0.0357",
                "5,55: 0.0333",
                "81,99: 0.0122",
                "21,36: 0.0109",
                "16,65: 0.0270",
                "6,8: 0.0278",
                "33,74: 0.0244",
                "13,37: 0.0204",
                "82,89: 0.0208",
                "21,50: 0.0120",
                "41,78: 0.0179",
                "57,93: 0.0116",
                "74,75: 0.0185",
                "32,95: 0.0294",
                "21,75: 0.0130",
                "54,75: 0.0123",
                "21,79: 0.0123",
                "41,50: 0.0189",
                "13,88: 0.0139",
                "71,87: 0.0217",
                "7,9: 0.0175",
                "14,99: 0.0238",
                "87,90: 0.0278",
                "5,72: 0.0345",
                "21,88: 0.0114",
                "8,46: 0.0244",
                "62,90: 0.0455",
                "38,67: 0.0143",
                "47,81: 0.0167",
                "16,17: 0.0556",
                "40,59: 0.0294",
                "39,90: 0.0435",
                "48,51: 0.1250",
                "47,85: 0.0167",
                "54,96: 0.0172",
                "22,90: 0.0244",
                "23,59: 0.0145",
                "88,91: 0.0111",
                "23,58: 0.0127",
                "39,77: 0.0435",
                "72,77: 0.0370",
                "55,95: 0.0213",
                "31,56: 0.0159",
                "38,97: 0.0161",
                "63,93: 0.0112",
                "39,71: 0.0303",
                "30,93: 0.0189"
            ]
        );
        assert_sort_eq!(
            Solution::compute_similarities(vec_vec![
                [14, 15, 100, 9, 3],
                [32, 1, 9, 3, 5],
                [15, 29, 2, 6, 8, 7],
                [7, 10]
            ]),
            ["0,1: 0.2500", "0,2: 0.1000", "2,3: 0.1429"]
        );
    }
}
