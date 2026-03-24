use std::{borrow::Cow, env, fmt::Display, str::FromStr};

pub fn read_str<'a>(var_name: &str, default: &'a str) -> Cow<'a, str> {
    match env::var(var_name) {
        Ok(value) => Cow::Owned(value),
        Err(env::VarError::NotPresent) => Cow::Borrowed(default),
        Err(env::VarError::NotUnicode(s)) => {
            eprintln!("Error reading {} (invalid UTF-8): {:?}", var_name, s);
            std::process::exit(1);
        }
    }
}

pub fn read_string(var_name: &str, default: &str) -> String {
    match env::var(var_name) {
        Ok(value) => value,
        Err(env::VarError::NotPresent) => String::from(default),
        Err(env::VarError::NotUnicode(s)) => {
            eprintln!("Error reading {} (invalid UTF-8): {:?}", var_name, s);
            std::process::exit(1);
        }
    }
}

pub trait Int: FromStr {}
impl Int for u16 {}
impl Int for u32 {}
impl Int for i16 {}
impl Int for i32 {}

pub fn read_num<T>(var_name: &str, default: T) -> T
where
    T: Int,
    T::Err: Display,
{
    match env::var(var_name) {
        Ok(value) => value.parse().unwrap_or_else(|e| {
            eprintln!("Error parsing {}: {}", var_name, e);
            std::process::exit(1);
        }),
        Err(env::VarError::NotPresent) => default,
        Err(env::VarError::NotUnicode(s)) => {
            eprintln!("Error reading {} (invalid UTF-8): {:?}", var_name, s);
            std::process::exit(1);
        }
    }
}
