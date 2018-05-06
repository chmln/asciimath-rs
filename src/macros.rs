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

// Thank you, https://github.com/bluss
#[macro_export]
macro_rules! scope {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(scope!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { scope!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = scope!(@count $($key),*);
            let mut _map = $crate::Scope::with_capacity(_cap);
            $(
                _map.set_var($key, $value);
            )*
            _map
        }
    };
}
