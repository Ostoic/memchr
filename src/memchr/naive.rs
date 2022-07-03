#![allow(dead_code)]

#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub fn memchr(n1: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == n1)
}

#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub fn memchr2(n1: u8, n2: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == n1 || b == n2)
}

#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub fn memchr3(n1: u8, n2: u8, n3: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == n1 || b == n2 || b == n3)
}

#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub fn memrchr(n1: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().rposition(|&b| b == n1)
}

#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub fn memrchr2(n1: u8, n2: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().rposition(|&b| b == n1 || b == n2)
}

#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub fn memrchr3(n1: u8, n2: u8, n3: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().rposition(|&b| b == n1 || b == n2 || b == n3)
}
