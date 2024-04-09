use async_trait::async_trait;
use cqrs_es::View;
use serde::{Deserialize, Serialize};

use crate::domain::tab::{aggregate::Tab, tab_id::TabId};

#[async_trait]
pub trait KitchenTodoListQuery: Sized {
    async fn get_kitchen_todo_list(&self) -> Vec<TodoListGroup>;
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct KitchenTodoList {
    inner: Vec<TodoListGroup>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TodoListGroup {
    pub(crate) tab_id: TabId,
    pub(crate) food_items: Vec<TodoListItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TodoListItem {
    pub(crate) menu_number: usize,
    pub(crate) description: String,
}

impl KitchenTodoList {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

impl TodoListGroup {
    pub fn tab_id(&self) -> TabId {
        self.tab_id
    }

    pub fn food_items(&self) -> Vec<TodoListItem> {
        let mut result = Vec::new();
        for item in self.food_items.iter() {
            result.push((*item).clone());
        }

        result
    }
}

impl TodoListItem {
    pub fn new(menu_number: usize, description: &str) -> Self {
        Self {
            menu_number,
            description: description.to_owned(),
        }
    }

    pub fn menu_number(&self) -> usize {
        self.menu_number
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}

impl View<Tab> for TodoListGroup {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Tab>) {
        match &event.payload {
            crate::domain::tab::event::TabEvent::FoodOrderPlaced { id, menu_item } => {
                let tab_item = TodoListItem {
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

impl View<Tab> for KitchenTodoList {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Tab>) {
        match &event.payload {
            crate::domain::tab::event::TabEvent::FoodOrderPlaced { id, menu_item } => {
                let mut todo_group = None;
                for group in self.inner.iter_mut() {
                    if group.tab_id == *id {
                        todo_group = Some(group);
                        break;
                    }
                }
                let tab_item = TodoListItem {
                    menu_number: menu_item.menu_number,
                    description: menu_item.description.clone(),
                };
                match todo_group {
                    Some(group) => group.food_items.push(tab_item),
                    None => self.inner.push(TodoListGroup {
                        tab_id: *id,
                        food_items: vec![tab_item],
                    }),
                };
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
    async fn get_kitchen_todo_list(&self) -> Vec<TodoListGroup> {
        // let inner = self.inner.read().await;
        let mut result = Vec::with_capacity(self.inner.len());
        for item in self.inner.iter() {
            result.push(item.clone())
        }

        result
    }
}

impl std::ops::Deref for KitchenTodoList {
    type Target = Vec<TodoListGroup>;

    fn deref(&self) -> &Self::Target {
        &self.inner
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
