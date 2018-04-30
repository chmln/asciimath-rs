#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($args:expr),*) => {};
}
