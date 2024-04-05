use super::tab_id::TabId;

#[derive(Debug, PartialEq)]
pub enum TabError {
    CannotCancelServedItem,
    TabHasUnservedItems,
    MustPayEnough,
    TabNotOpened,
    DrinkNotOutstanding { menu_number: usize },
    TabIsOpen { id: TabId },
}

impl std::error::Error for TabError {}

impl std::fmt::Display for TabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            TabError::CannotCancelServedItem => String::from("cannot cancel served item"),
            TabError::TabHasUnservedItems => String::from("tab has unserved items"),
            TabError::MustPayEnough => String::from("payment amount is not enough"),
            TabError::TabNotOpened => String::from("tab is not open"),
            TabError::DrinkNotOutstanding { menu_number } => {
                format!("drink was not served: menu number {menu_number}")
            }
            TabError::TabIsOpen { id } => format!("already open: id: {id}"),
        };

        write!(f, "tab error: {msg}")
    }
}
