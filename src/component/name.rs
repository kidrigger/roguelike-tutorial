use std::fmt::Display;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Name(pub String);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for Name
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Name {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }
}
