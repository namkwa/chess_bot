use std::collections::HashSet;

#[derive(PartialEq)]
pub struct ChecksAndPins {
    pub destinations: HashSet<(usize, usize)>,
    pub is_checked: bool,
    pub current_positions: Option<HashSet<(usize, usize)>>,
    pub is_pinned: Option<bool>,
}
