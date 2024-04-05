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
        todo!()
    }

    fn event_version(&self) -> String {
        todo!()
    }
}
