use cqrs_es::DomainEvent;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::waiter_id::WaiterId;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrderItem {
    pub menu_number: usize,
    pub description: String,
    pub is_drink: bool,
    pub price: Decimal,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TabEvent {
    TabOpened { waiter_id: WaiterId, table: usize },
    ItemOrdered { order_item: OrderItem },
}

impl DomainEvent for TabEvent {
    fn event_type(&self) -> String {
        todo!()
    }

    fn event_version(&self) -> String {
        todo!()
    }
}
