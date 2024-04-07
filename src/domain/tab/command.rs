use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{tab_id::TabId, waiter_id::WaiterId};

#[derive(Debug, Deserialize)]
pub enum TabCommand {
    OpenTab {
        id: TabId,
        waiter_id: WaiterId,
        table: usize,
    },
    PlaceOrder {
        order_items: Vec<OrderItem>,
    },
    MarkDrinksServed {
        id: TabId,
        menu_numbers: Vec<usize>,
    },
    MarkFoodPrepared {
        id: TabId,
        menu_numbers: Vec<usize>,
    },
    MarkFoodServed {
        id: TabId,
        menu_numbers: Vec<usize>,
    },
    CloseTab {
        id: TabId,
        amount_paid: Decimal,
    },
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OrderItem {
    pub menu_number: usize,
    pub description: String,
    pub is_drink: bool,
    pub price: Decimal,
}
