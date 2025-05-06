// TODO: at some point remove size and row_label
// for non-columns. It's a waste of space for most of the nodes.
pub(super) struct Node {
    pub left: usize,
    pub right: usize,
    pub up: usize,
    pub down: usize,
    pub col: usize,
    pub row_label: usize,
    pub size: usize,
}
