use cqrs_es::View;
use serde::{Deserialize, Serialize};

use crate::domain::tab::{aggregate::Tab, tab_id::TabId};

// pub trait OpenTabQuery {
//     fn active_table_numbers(&self) -> Vec<usize>;
//     fn invoice_for_table(&self, table: usize) -> TabInvoice;
//     fn tab_for_table(&self, table: usize) -> TabStatus;
//     fn waiter_todo_list(&self, id: WaiterId) -> WaiterTodoList;
// }

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WaiterTodoList {
    inner: Vec<OpenTab>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OpenItem {
    menu_number: usize,
    description: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

    pub fn remove_item(&mut self, menu_number: usize) {
        self.open_items.remove(
            self.open_items
                .iter()
                .position(|i| i.menu_number == menu_number)
                .unwrap(),
        );
    }
}

impl WaiterTodoList {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

impl std::ops::Deref for WaiterTodoList {
    type Target = Vec<OpenTab>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl View<Tab> for WaiterTodoList {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Tab>) {
        match &event.payload {
            crate::domain::tab::event::TabEvent::FoodOrderPlaced {
                id: _,
                menu_item: _,
            } => {}
            crate::domain::tab::event::TabEvent::FoodPrepared {
                id: _,
                menu_number: _,
            } => {}
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::tab::{
        queries::open_tabs::{OpenItem, WaiterTodoList},
        tab_id::TabId,
    };

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

    #[test]
    fn given_open_tab_with_one_item_when_remove_item_then_open_items_is_empty() {
        // Arrange
        let id = TabId::new();
        let mut tab = OpenTab::new(id);
        tab.add_item(OpenItem {
            menu_number: 2,
            description: "Coca-Cola".into(),
        });

        // Act
        tab.remove_item(2);

        let tab = tab;
        assert!(tab.open_items().is_empty())
    }

    #[test]
    #[allow(non_snake_case)]
    fn when_new_WaiterTodoList_then_open_items_is_empty() {
        let list = WaiterTodoList::new();

        assert!(list.is_empty())
    }
}
