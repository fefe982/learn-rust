pub fn make_string_vec<const N: usize>(arr_in: [&str; N]) -> Vec<String> {
    arr_in.into_iter().map(|x| String::from(x)).collect()
}
