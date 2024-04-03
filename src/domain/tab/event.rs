use cqrs_es::DomainEvent;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::waiter_id::WaiterId;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FoodItem {
    pub menu_number: usize,
    pub description: String,
    pub price: Decimal,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DrinkItem {
    pub menu_number: usize,
    pub description: String,
    pub price: Decimal,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TabEvent {
    TabOpened { waiter_id: WaiterId, table: usize },
    FoodOrderPlaced { food_item: FoodItem },
    DrinkOrderPlaced { drink_item: DrinkItem },
}

impl DomainEvent for TabEvent {
    fn event_type(&self) -> String {
        todo!()
    }

    fn event_version(&self) -> String {
        todo!()
    }
}
