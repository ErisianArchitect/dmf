pub mod parsing;
pub mod trie;

#[must_use]
#[inline(always)]
pub const fn or_empty(condition: bool, s: &str) -> &str {
    if condition {
        s
    } else {
        ""
    }
}

#[must_use]
#[inline(always)]
pub const fn const_or_empty<const CONDITION: bool>(s: &str) -> &str {
    crate::functional::const_select_ref::<CONDITION, _>(s, "")
}