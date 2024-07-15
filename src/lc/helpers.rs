use std::ops::Index;
pub fn make_string_vec<const N: usize>(arr_in: [&str; N]) -> Vec<String> {
    arr_in.into_iter().map(|x| String::from(x)).collect()
}

#[macro_export]
macro_rules! vec_str {
    ($($x:expr),*$(,)?) => (vec![$(String::from($x)),*]);
}
#[macro_export]
macro_rules! vec_vec {
    ($([$($x:expr),*$(,)?]),*$(,)?) => (vec![$(vec![$($x),*]),*]);
}
#[macro_export]
macro_rules! vec_vec_str {
    ($($x:tt),*$(,)?) => (vec![$(vec_str!$x),*]);
}
#[macro_export]
macro_rules! vec_chr {
    ($($x:expr),*$(,)?) => (vec![$($x.chars().next().unwrap()),*]);
}
#[macro_export]
macro_rules! vec_vec_chr {
    ($($x:tt),*$(,)?) => (vec![$(vec_chr!$x),*]);
}
#[macro_export]
macro_rules! assert_sort_eq {
    ($left:expr, $right:expr) => {
        match ($left, $right) {
            (mut left_val, mut right_val) => {
                left_val.sort_unstable();
                right_val.sort_unstable();
                assert_eq!(left_val, right_val);
            }
        }
    };
}

pub enum Any {
    Null,
    Char(char),
    Str(&'static str),
    I32(i32),
    Bool(bool),
    Vec(Vec<Any>),
}
impl From<()> for Any {
    fn from(_: ()) -> Self {
        Any::Null
    }
}
impl From<char> for Any {
    fn from(x: char) -> Self {
        Any::Char(x)
    }
}
impl From<&'static str> for Any {
    fn from(x: &'static str) -> Self {
        Any::Str(x)
    }
}
impl From<i32> for Any {
    fn from(x: i32) -> Self {
        Any::I32(x)
    }
}
impl From<Vec<Any>> for Any {
    fn from(x: Vec<Any>) -> Self {
        Any::Vec(x)
    }
}
impl From<bool> for Any {
    fn from(x: bool) -> Self {
        Any::Bool(x)
    }
}
impl Any {
    pub fn as_char(&self) -> char {
        match self {
            Any::Char(x) => *x,
            _ => panic!(),
        }
    }
    pub fn as_i32(&self) -> i32 {
        match self {
            Any::I32(x) => *x,
            _ => panic!(),
        }
    }
    pub fn as_string(&self) -> String {
        match self {
            Any::Str(x) => x.to_string(),
            _ => panic!(),
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            Any::Bool(x) => *x,
            _ => panic!(),
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Any::Str(x) => *x,
            _ => panic!(),
        }
    }
    pub fn as_slice(&self) -> &[Any] {
        match self {
            Any::Vec(x) => &x[..],
            _ => panic!(),
        }
    }
    pub fn as_vec_char(&self) -> Vec<char> {
        match self {
            Any::Vec(x) => x.iter().map(|x| x.as_char()).collect::<Vec<_>>(),
            _ => panic!(),
        }
    }
    pub fn as_vec_i32(&self) -> Vec<i32> {
        match self {
            Any::Vec(x) => x.iter().map(|x| x.as_i32()).collect::<Vec<_>>(),
            _ => panic!(),
        }
    }
    pub fn as_vec_string(&self) -> Vec<String> {
        match self {
            Any::Vec(x) => x.iter().map(|x| x.as_string()).collect::<Vec<_>>(),
            _ => panic!(),
        }
    }
}
impl Index<usize> for Any {
    type Output = Any;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Any::Vec(x) => &x[index],
            _ => panic!(),
        }
    }
}
#[macro_export]
macro_rules! vec_any {
    ($($x:tt),*$(,)?) => (vec![$(any_cast!($x)),*]);
}

#[macro_export]
macro_rules! any_cast {
    ([$($x:tt),*$(,)?]) => (any_cast!(vec![$(any_cast!($x)),*]));
    ($x:expr) => ($crate::lc::helpers::Any::from($x));
}
