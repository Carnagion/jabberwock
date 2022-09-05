#[macro_export]
macro_rules! hatter_error
{
    ($kind:ident, $details:expr) =>
    {
        {
            use hatter::Error;
            use hatter::ErrorKind;
            Error::new(ErrorKind::$kind, $details.into(), 0, 0)
        }
    };
}

pub use hatter_error;

#[macro_export]
macro_rules! require_env_var
{
    ($var:expr, $env:expr) =>
    {
        {
            use hatter::Error;
            use hatter::ErrorKind;
            $env.lookup($var)
                .map_or_else(|| Err(Error::new(ErrorKind::RuntimeError, format!("Missing variable in env: {}", $var), 0, 0)), Ok)
        }
    };
}

pub use require_env_var;