use super::tab_id::TabId;

#[derive(Debug, PartialEq)]
pub enum TabError {
    CannotCancelServedItem,
    TabHasUnservedItems,
    MustPayEnough,
    TabNotOpened,
    DrinkNotOutstanding { menu_number: usize },
    TabIsOpen { id: TabId },
    FoodNotOutstanding { menu_number: usize },
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
                format!("drink is not outstanding: menu number {menu_number}")
            }
            TabError::TabIsOpen { id } => format!("already open: {id}"),
            TabError::FoodNotOutstanding { menu_number } => {
                format!("food is not outstanding: menu number {menu_number}")
            }
        };

        write!(f, "tab error: {msg}")
    }
}

#[cfg(test)]
pub mod tests {
    use crate::domain::tab::tab_id::TabId;

    use super::TabError;

    #[test]
    fn error_to_string() {
        assert_eq!(
            format!("{}", TabError::CannotCancelServedItem),
            "tab error: cannot cancel served item"
        );
        assert_eq!(
            format!("{}", TabError::DrinkNotOutstanding { menu_number: 1 }),
            "tab error: drink is not outstanding: menu number 1"
        );
        assert_eq!(
            format!("{}", TabError::MustPayEnough),
            "tab error: payment amount is not enough"
        );
        assert_eq!(
            format!("{}", TabError::TabHasUnservedItems),
            "tab error: tab has unserved items"
        );
        assert_eq!(
            format!(
                "{}",
                TabError::TabIsOpen {
                    id: TabId::default()
                }
            ),
            "tab error: already open: 00000000-0000-0000-0000-000000000000"
        );
    }
}
