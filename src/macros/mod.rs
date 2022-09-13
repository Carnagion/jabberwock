#[macro_export]
macro_rules! error {
    ($kind:ident, $details:expr) => {
        {
            use hatter::Error;
            use hatter::ErrorKind;
            Error::new(ErrorKind::$kind, $details.into(), 0, 0)
        }
    };
}

pub(crate) use error;

#[macro_export]
macro_rules! require_env_string {
    ($var:expr, $env:expr) => {
        {
            use hatter::Error;
            use hatter::ErrorKind;
            use hatter::Value;
            let var = $env.lookup($var)
                .map_or_else(|| Err(Error::new(ErrorKind::RuntimeError, format!("Missing variable in env: {}", $var), 0, 0)), Ok)?
                .to_owned();
            match var {
                Value::String(symbol) => Ok(symbol),
                value => Err(Error::new(ErrorKind::RuntimeError, format!("Expected string in \"{}\", got {}", $var, value.typename()), 0, 0)),
            }
        }
    };
}

pub(crate) use require_env_string;