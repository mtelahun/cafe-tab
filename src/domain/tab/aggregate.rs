use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    command::{OrderItem, TabCommand},
    error::TabError,
    event::{MenuItem, TabEvent},
    services::TabServices,
    tab_id::TabId,
    waiter_id::WaiterId,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tab {
    id: TabId,
    table: usize,
    opened: bool,
    waiter_id: WaiterId,
    food_item: MenuItem,
    drink_item: MenuItem,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Waiter {
    id: Uuid,
    name: String,
}

#[async_trait]
impl Aggregate for Tab {
    type Command = TabCommand;
    type Event = TabEvent;
    type Error = TabError;
    type Services = TabServices;

    fn aggregate_type() -> String {
        "Tab".into()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            TabCommand::OpenTab { waiter_id, table } => self.trigger_open_event(&waiter_id, table),
            TabCommand::PlaceOrder { order_items } => {
                self.read_orders_and_trigger_events(&order_items)
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            TabEvent::TabOpened { waiter_id, table } => {
                self.opened = true;
                self.waiter_id = waiter_id;
                self.table = table;
            }
            TabEvent::FoodOrderPlaced { menu_item } => self.food_item = menu_item,
            TabEvent::DrinkOrderPlaced { menu_item } => self.drink_item = menu_item,
        }
    }
}

impl Tab {
    fn read_orders_and_trigger_events(
        &self,
        order_items: &[OrderItem],
    ) -> Result<Vec<TabEvent>, TabError> {
        let mut orders = Vec::new();
        for order_item in order_items.iter() {
            let menu_item = MenuItem {
                menu_number: order_item.menu_number,
                description: order_item.description.to_owned(),
                price: order_item.price,
            };
            if self.opened {
                if order_item.is_drink {
                    orders.push(TabEvent::DrinkOrderPlaced { menu_item });
                } else {
                    orders.push(TabEvent::FoodOrderPlaced { menu_item });
                }
            } else {
                return Err(TabError::TabNotOpened);
            }
        }

        Ok(orders)
    }

    fn trigger_open_event(
        &self,
        waiter_id: &WaiterId,
        table: usize,
    ) -> Result<Vec<TabEvent>, TabError> {
        Ok(vec![TabEvent::TabOpened {
            waiter_id: *waiter_id,
            table,
        }])
    }
}

#[cfg(test)]
pub mod tests {
    use cqrs_es::test::TestFramework;
    use rust_decimal::Decimal;

    use crate::domain::tab::{
        aggregate::Tab,
        command::{OrderItem, TabCommand},
        error::TabError,
        event::{MenuItem, TabEvent},
        services::TabServices,
        waiter_id::WaiterId,
    };

    #[test]
    #[allow(non_snake_case)]
    fn given_unopened_tab_when_any_command_then_TabNotOpened_error() {
        // Arrange
        let tab_services = TabServices {};
        let executor = TestFramework::<Tab>::with(tab_services).given_no_previous_events();

        // Act
        let result = executor
            .when(TabCommand::PlaceOrder {
                order_items: vec![OrderItem::default()],
            })
            .inspect_result();

        // Assert
        assert_eq!(result.err().unwrap(), TabError::TabNotOpened)
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_tab_with_no_events_when_OpenTab_command_then_TabOpened_event() {
        // Arrange
        let waiter_id = WaiterId::new();
        let tab_services = TabServices {};
        let executor = TestFramework::<Tab>::with(tab_services).given_no_previous_events();

        // Act
        let result = executor.when(TabCommand::OpenTab {
            waiter_id,
            table: 1,
        });
        let mut event = result
            .inspect_result()
            .expect("failed to execute command: OpenTab");

        // Assert
        let event = event.pop().unwrap();
        assert_eq!(
            event,
            TabEvent::TabOpened {
                waiter_id,
                table: 1
            }
        )
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_opened_tab_when_order_food_then_ItemOrdered_event() {
        // Arrange
        let waiter_id = WaiterId::new();
        let tab_services = TabServices {};
        let order_items = vec![OrderItem {
            menu_number: 1,
            description: "Steak".into(),
            is_drink: false,
            price: Decimal::from(10),
        }];
        let executor = TestFramework::<Tab>::with(tab_services).given(vec![TabEvent::TabOpened {
            waiter_id,
            table: 1,
        }]);

        // Act
        let mut event = executor
            .when(TabCommand::PlaceOrder { order_items })
            .inspect_result()
            .expect("failed to execute command: OrderItem");

        // Assert
        assert_eq!(event.len(), 1);
        let event = event.pop().unwrap();
        assert_eq!(
            event,
            TabEvent::FoodOrderPlaced {
                menu_item: MenuItem {
                    menu_number: 1,
                    description: "Steak".into(),
                    price: Decimal::from(10),
                }
            },
            "ItemOrdered"
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_opened_tab_when_order_1_drink_then_ItemOrdered_event() {
        // Arrange
        let waiter_id = WaiterId::new();
        let tab_services = TabServices {};
        let order_items = vec![OrderItem {
            menu_number: 2,
            description: "Coca-Cola".into(),
            is_drink: true,
            price: Decimal::from(3),
        }];
        let executor = TestFramework::<Tab>::with(tab_services).given(vec![TabEvent::TabOpened {
            waiter_id,
            table: 1,
        }]);

        // Act
        let mut event = executor
            .when(TabCommand::PlaceOrder { order_items })
            .inspect_result()
            .expect("failed to execute command: OrderItem");

        // Assert
        assert_eq!(event.len(), 1);
        let event = event.pop().unwrap();
        assert_eq!(
            event,
            TabEvent::DrinkOrderPlaced {
                menu_item: MenuItem {
                    menu_number: 2,
                    description: "Coca-Cola".into(),
                    price: Decimal::from(3),
                }
            },
            "DrinkOrderPlaced"
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_opened_tab_when_order_multiple_items_then_multiple_OrderPlaced_events() {
        // Arrange
        let waiter_id = WaiterId::new();
        let tab_services = TabServices {};
        let order_items = vec![
            OrderItem {
                menu_number: 1,
                description: "Steak".into(),
                is_drink: false,
                price: Decimal::from(10),
            },
            OrderItem {
                menu_number: 2,
                description: "Coca-Cola".into(),
                is_drink: true,
                price: Decimal::from(3),
            },
        ];
        let executor = TestFramework::<Tab>::with(tab_services).given(vec![TabEvent::TabOpened {
            waiter_id,
            table: 1,
        }]);

        // Act
        let event = executor
            .when(TabCommand::PlaceOrder { order_items })
            .inspect_result()
            .expect("failed to execute command: OrderItem");

        // Assert
        assert_eq!(event.len(), 2);
        assert_eq!(
            event[0],
            TabEvent::FoodOrderPlaced {
                menu_item: MenuItem {
                    menu_number: 1,
                    description: "Steak".into(),
                    price: Decimal::from(10),
                }
            },
            "FoodOrderPlaced"
        );
        assert_eq!(
            event[1],
            TabEvent::DrinkOrderPlaced {
                menu_item: MenuItem {
                    menu_number: 2,
                    description: "Coca-Cola".into(),
                    price: Decimal::from(3),
                }
            },
            "DrinkOrderPlaced"
        );
    }
}
