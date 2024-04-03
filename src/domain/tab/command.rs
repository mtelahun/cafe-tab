use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::waiter_id::WaiterId;

#[derive(Debug, Deserialize)]
pub enum TabCommand {
    OpenTab { waiter_id: WaiterId, table: usize },
    PlaceOrder { order_items: Vec<OrderItem> },
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OrderItem {
    pub menu_number: usize,
    pub description: String,
    pub is_drink: bool,
    pub price: Decimal,
}
