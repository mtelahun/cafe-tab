use serde::Deserialize;

use super::{event::OrderItem, waiter_id::WaiterId};

#[derive(Debug, Deserialize)]
pub enum TabCommand {
    OpenTab { waiter_id: WaiterId, table: usize },
    PlaceOrder { order_item: OrderItem },
}
