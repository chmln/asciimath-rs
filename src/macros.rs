macro_rules! debug {
    ($( $args:expr ),*) => {
        if cfg!(debug_assertions) {
            println!( $( $args ),* );
        }
    }
}

// TODO: micro-optimizations / use with_capacity() somehow
macro_rules! vec_deque {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = $crate::std::collections::VecDeque::new();
            $(
                temp_vec.push_back($x);
            )*
            temp_vec
        }
    };
}
