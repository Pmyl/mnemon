//! Common types used across the application

/// Navigation direction for mnemon transitions
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Direction {
    /// Navigate forward (left-to-right slide)
    #[default]
    Forward,
    /// Navigate backward (right-to-left slide)
    Backward,
}
