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
    ($([$($x:expr),*$(,)?]),*$(,)?) => (
        vec![$(vec![$($x),*]),*]
    );
}
#[macro_export]
macro_rules! vec_vec_str {
    ($($x:tt),*) => (
        [$(vec_str!$x),*].to_vec()
    );
    ($($x:expr,)*) => (vec![$($x),*])
}
#[macro_export]
macro_rules! vec_chr {
    ($($x:expr),*) => (
        [$($x.chars().nth(0).unwrap()),*].to_vec()
    );
    ($($x:expr,)*) => (vec![$($x),*])
}
#[macro_export]
macro_rules! vec_vec_chr {
    ($($x:tt),*) => (
        [$(vec_chr!$x),*].to_vec()
    );
    ($($x:expr,)*) => (vec![$($x),*])
}
