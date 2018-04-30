#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($args:expr),*) => {};
}

// TODO: micro-optimizations / use with_capacity() somehow
#[macro_export]
macro_rules! vec_deque {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = VecDeque::new();
            $(
                temp_vec.push_back($x);
            )*
            temp_vec
        }
    };
}
