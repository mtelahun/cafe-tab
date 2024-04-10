use crate::domain::tab::tab_id::TabId;

#[derive(Clone, Debug)]
pub struct OpenItem {
    menu_number: usize,
    description: String,
}

#[derive(Clone, Debug, Default)]
pub struct OpenTab {
    id: TabId,
    open_items: Vec<OpenItem>,
}

impl OpenItem {
    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn menu_number(&self) -> usize {
        self.menu_number
    }
}

impl OpenTab {
    pub fn new(id: TabId) -> Self {
        Self {
            id,
            open_items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: OpenItem) {
        self.open_items.push(item)
    }

    pub fn id(&self) -> TabId {
        self.id
    }

    pub fn open_items(&self) -> Vec<OpenItem> {
        self.open_items.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::domain::tab::{queries::open_tabs::OpenItem, tab_id::TabId};

    use super::OpenTab;

    #[test]
    fn when_new_tab_then_open_items_is_empty() {
        let tab = OpenTab::new(TabId::default());

        assert!(tab.open_items.is_empty())
    }

    #[test]
    fn given_open_tab_when_add_open_item_then_open_items_has_one_item() {
        // Arrange
        let id = TabId::new();
        let mut tab = OpenTab::new(id);
        let item = OpenItem {
            menu_number: 2,
            description: "Coca-Cola".into(),
        };

        // Act
        tab.add_item(item);

        let tab = tab;
        assert_eq!(tab.id(), id);
        assert_eq!(tab.open_items.len(), 1);
        assert_eq!(tab.open_items()[0].menu_number(), 2);
        assert_eq!(tab.open_items()[0].description(), "Coca-Cola");
    }
}
