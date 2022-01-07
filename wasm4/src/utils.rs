use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OutOfDomainError(pub(crate) ());

impl fmt::Display for OutOfDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("input is outside the domain")
    }
}

impl std::error::Error for OutOfDomainError {}

pub(crate) const fn checked_add_pairs(lhs: [u32; 2], rhs: [u32; 2]) -> Option<[u32; 2]> {
    Some([
        match lhs[0].checked_add(rhs[0]) {
            Some(it) => it,
            None => return None,
        },
        match lhs[1].checked_add(rhs[1]) {
            Some(it) => it,
            None => return None,
        },
    ])
}

pub(crate) const fn lt_pairs(lhs: [u32; 2], rhs: [u32; 2]) -> bool {
    lhs[0] < rhs[0] && lhs[1] < rhs[1]
}

pub(crate) const fn le_pairs(lhs: [u32; 2], rhs: [u32; 2]) -> bool {
    lhs[0] <= rhs[0] && lhs[1] <= rhs[1]
}
