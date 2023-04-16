pub fn make_string_vec<const N: usize>(arr_in: [&str; N]) -> Vec<String> {
    arr_in.into_iter().map(|x| String::from(x)).collect()
}

#[macro_export]
macro_rules! vec_str {
    ($($x:expr),*) => (
        [$(String::from($x)),*].to_vec()
    );
    ($($x:expr,)*) => (vec![$($x),*])
}
#[macro_export]
macro_rules! vec_vec {
    ($($x:expr),*) => (
        [$($x.to_vec()),*].to_vec()
    );
    ($($x:expr,)*) => (vec![$($x),*])
}
