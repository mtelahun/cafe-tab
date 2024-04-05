#![allow(unused_variables)]
use cqrs_es::DomainEvent;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{tab_id::TabId, waiter_id::WaiterId};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MenuItem {
    pub menu_number: usize,
    pub description: String,
    pub price: Decimal,
    pub quantity: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TabEvent {
    TabOpened {
        id: TabId,
        waiter_id: WaiterId,
        table: usize,
    },
    FoodOrderPlaced {
        id: TabId,
        menu_item: MenuItem,
    },
    DrinkOrderPlaced {
        id: TabId,
        menu_item: MenuItem,
    },
    DrinkServed {
        id: TabId,
        menu_number: usize,
    },
}

impl DomainEvent for TabEvent {
    fn event_type(&self) -> String {
        match self {
            TabEvent::TabOpened {
                id,
                waiter_id,
                table,
            } => format!(
                "TabEvent::TabOpened {{ id: {id}, waiter_id: {waiter_id}, table: {table} }}"
            ),
            TabEvent::FoodOrderPlaced { id, menu_item } => {
                format!("TabEvent::FoodOrderPlaced {{ id: {id}, menu_item: {menu_item:?} }}")
            }
            TabEvent::DrinkOrderPlaced { id, menu_item } => {
                format!("TabEvent::DrinkOrderPlaced {{ id: {id}, menu_item: {menu_item:?} }}")
            }
            TabEvent::DrinkServed { id, menu_number } => {
                format!("TabEvent::DrinkServed {{ id: {id}, menu_number: 1 }}")
            }
        }
    }

    fn event_version(&self) -> String {
        "1.0".into()
    }
}

#[cfg(test)]
mod tests {
    use cqrs_es::DomainEvent;
    use rust_decimal::Decimal;

    use crate::domain::tab::{tab_id::TabId, waiter_id::WaiterId};

    use super::{MenuItem, TabEvent};

    #[test]
    #[allow(non_snake_case)]
    fn event_type() {
        let id = TabId::new();
        let waiter_id = WaiterId::new();
        let menu_item = MenuItem {
            menu_number: 1,
            description: "MenuItem".into(),
            price: Decimal::ZERO,
            quantity: 0,
        };
        let event1 = TabEvent::DrinkOrderPlaced {
            id,
            menu_item: menu_item.clone(),
        };
        let event2 = TabEvent::DrinkServed { id, menu_number: 1 };
        let event3 = TabEvent::FoodOrderPlaced {
            id,
            menu_item: menu_item.clone(),
        };
        let event4 = TabEvent::TabOpened {
            id,
            waiter_id,
            table: 1,
        };

        assert_eq!(
            event1.event_type(),
            format!("TabEvent::DrinkOrderPlaced {{ id: {id}, menu_item: {menu_item:?} }}"),
        );
        assert_eq!(
            event2.event_type(),
            format!("TabEvent::DrinkServed {{ id: {id}, menu_number: 1 }}"),
        );
        assert_eq!(
            event3.event_type(),
            format!("TabEvent::FoodOrderPlaced {{ id: {id}, menu_item: {menu_item:?} }}"),
        );
        assert_eq!(
            event4.event_type(),
            format!("TabEvent::TabOpened {{ id: {id}, waiter_id: {waiter_id}, table: 1 }}"),
        );
    }

    #[test]
    fn event_version_is_1_0() {
        let id = TabId::new();
        let waiter_id = WaiterId::new();
        let menu_item = MenuItem {
            menu_number: 1,
            description: "MenuItem".into(),
            price: Decimal::ZERO,
            quantity: 0,
        };
        let event1 = TabEvent::DrinkOrderPlaced {
            id,
            menu_item: menu_item.clone(),
        };
        let event2 = TabEvent::DrinkServed { id, menu_number: 1 };
        let event3 = TabEvent::FoodOrderPlaced {
            id,
            menu_item: menu_item.clone(),
        };
        let event4 = TabEvent::TabOpened {
            id,
            waiter_id,
            table: 1,
        };

        assert_eq!(event1.event_version(), String::from("1.0"));
        assert_eq!(event1.event_version(), event2.event_version(),);
        assert_eq!(event2.event_version(), event3.event_version(),);
        assert_eq!(event3.event_version(), event4.event_version(),);
    }
}
