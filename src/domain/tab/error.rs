#[derive(Debug, PartialEq)]
pub enum TabError {
    CannotCancelServedItem,
    TabHasUnservedItems,
    MustPayEnough,
    TabNotOpened,
}

impl std::error::Error for TabError {}

impl std::fmt::Display for TabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            TabError::CannotCancelServedItem => "cannot cancel served item",
            TabError::TabHasUnservedItems => "tab has unserved items",
            TabError::MustPayEnough => "payment amount is not enough",
            TabError::TabNotOpened => "tab is not open",
        };

        write!(f, "tab error: {msg}")
    }
}
