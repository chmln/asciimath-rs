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
