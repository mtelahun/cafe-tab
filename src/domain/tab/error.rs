#[derive(Debug, PartialEq)]
pub enum TabError {
    CannotCancelServedItem,
    TabHasUnservedItems,
    MustPayEnough,
    TabNotOpened,
    DrinkWasNotServed { menu_number: usize },
}

impl std::error::Error for TabError {}

impl std::fmt::Display for TabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            TabError::CannotCancelServedItem => String::from("cannot cancel served item"),
            TabError::TabHasUnservedItems => String::from("tab has unserved items"),
            TabError::MustPayEnough => String::from("payment amount is not enough"),
            TabError::TabNotOpened => String::from("tab is not open"),
            TabError::DrinkWasNotServed { menu_number } => {
                format!("drink was not served: menu number {menu_number}")
            }
        };

        write!(f, "tab error: {msg}")
    }
}
