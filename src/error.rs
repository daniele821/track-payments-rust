use std::error::Error as StdError;

pub enum Error {
    Generic(Box<dyn StdError>),
}

impl<T: StdError + 'static> From<T> for Error {
    fn from(value: T) -> Self {
        Self::Generic(Box::new(value))
    }
}
