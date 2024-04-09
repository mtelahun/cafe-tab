use async_trait::async_trait;
use cqrs_es::View;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::tab::{aggregate::Tab, tab_id::TabId};

#[async_trait]
pub trait KitchenTodoListQuery: Sized {
    async fn get_kitchen_todo_list(&self) -> Vec<KitchenTabView>;
}

#[derive(Debug, Default)]
pub struct KitchenTodoList {
    inner: RwLock<Vec<KitchenTabView>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct KitchenTabView {
    pub(crate) tab_id: TabId,
    pub(crate) food_items: Vec<KitchenTabItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct KitchenTabItem {
    pub(crate) menu_number: usize,
    pub(crate) description: String,
}

impl KitchenTodoList {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(Vec::new()),
        }
    }
}

impl KitchenTabView {
    pub fn tab_id(&self) -> TabId {
        self.tab_id
    }

    pub fn food_items(&self) -> Vec<KitchenTabItem> {
        let mut result = Vec::new();
        for item in self.food_items.iter() {
            result.push((*item).clone());
        }

        result
    }
}

impl KitchenTabItem {
    pub fn new(menu_number: usize, description: &str) -> Self {
        Self {
            menu_number,
            description: description.to_owned(),
        }
    }
}

impl View<Tab> for KitchenTabView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Tab>) {
        match &event.payload {
            crate::domain::tab::event::TabEvent::FoodOrderPlaced { id, menu_item } => {
                let tab_item = KitchenTabItem {
                    menu_number: menu_item.menu_number,
                    description: menu_item.description.clone(),
                };
                self.tab_id = *id;
                self.food_items.push(tab_item);
            }
            crate::domain::tab::event::TabEvent::FoodPrepared {
                id: _,
                menu_number: _,
            } => {}
            _ => {}
        }
    }
}

#[async_trait]
impl KitchenTodoListQuery for KitchenTodoList {
    async fn get_kitchen_todo_list(&self) -> Vec<KitchenTabView> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::tab::queries::kitchen::KitchenTodoListQuery;

    use super::KitchenTodoList;

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn given_new_KitchenTodoList_then_it_is_empty() {
        let list = KitchenTodoList::new();

        assert!(list.get_kitchen_todo_list().await.is_empty())
    }
}
