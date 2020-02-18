use failure::Fail;

macro_rules! from_error {
    ($type:ty, $target:ident, $targetvar:expr) => {
        impl From<$type> for $target {
            fn from(s: $type) -> Self {
                $targetvar(s.into())
            }
        }
    };
}

#[derive(Debug, Fail)]
pub enum DbusParseError {
    #[fail(display = "Unknown error")]
    UnknownError,
}
