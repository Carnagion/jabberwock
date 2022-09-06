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

pub(crate) use hatter_error;

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

pub(crate) use require_env_var;

macro_rules! require_env_string
{
    ($var:expr, $env:expr) =>
    {
        {
            use hatter::Value;
            use crate::utils::macros;
            match macros::require_env_var!($var, $env)?.to_owned()
            {
                Value::String(var) => Ok(var),
                val => Err(macros::hatter_error!(RuntimeError, format!("Expected string in {}, got: {}", $var, val.typename()))),
            }
        }
    };
}

pub(crate) use require_env_string;