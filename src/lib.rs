pub mod css;
pub mod html;
pub mod io;
pub mod js;
pub mod parser;

#[macro_export]
macro_rules! out {
    ($($arg:tt)*) => {{
        if cfg!(not(windows)) {
            println!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {{
        if cfg!(not(windows)) {
            eprintln!($($arg)*);
        }
    }};
}
