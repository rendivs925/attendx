use std::sync::LazyLock;

macro_rules! env_var {
    ($name:ident, $default:expr) => {
        pub static $name: LazyLock<&'static str> =
            LazyLock::new(|| option_env!(stringify!($name)).unwrap_or($default));
    };
}

env_var!(API_BASE_URL, "http://localhost:8000");
