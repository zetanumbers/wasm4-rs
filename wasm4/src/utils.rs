use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OutOfDomainError(pub(crate) ());

impl fmt::Display for OutOfDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("input is outside the domain")
    }
}

impl std::error::Error for OutOfDomainError {}
