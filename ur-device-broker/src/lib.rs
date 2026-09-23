//! Platform device access boundary.

/// Returns whether this build targets Windows device APIs.
pub const fn is_windows() -> bool {
    cfg!(windows)
}
