use std::collections::HashMap;

use async_trait::async_trait;
use cqrs_es::Aggregate;
use rust_decimal::Decimal;
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
    food_items: Vec<MenuItem>,
    foods_prepared: HashMap<usize, usize>,
    foods_served: HashMap<usize, usize>,
    drink_items: Vec<MenuItem>,
    drinks_served: HashMap<usize, usize>,
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
            TabCommand::OpenTab { waiter_id, table } => {
                if self.tab_is_open() {
                    return Err(TabError::TabIsOpen { id: self.id });
                }
                self.handle_open_tab_command(&waiter_id, table)
            }
            TabCommand::CloseTab { id, amount_paid } => {
                self.tab_is_open_or_error()?;
                self.handle_close_tab_command(id, amount_paid)
            }
            TabCommand::PlaceOrder { order_items } => {
                self.tab_is_open_or_error()?;
                self.handle_place_order_command(&order_items)
            }
            TabCommand::MarkDrinksServed { id, menu_numbers } => {
                self.tab_is_open_or_error()?;
                self.handle_mark_drink_served_command(id, menu_numbers)
            }
            TabCommand::MarkFoodPrepared { id, menu_numbers } => {
                self.tab_is_open_or_error()?;
                self.handle_mark_food_prepared_command(id, &menu_numbers)
            }
            TabCommand::MarkFoodServed { id, menu_numbers } => {
                self.tab_is_open_or_error()?;
                self.handle_mark_food_served_command(id, &menu_numbers)
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            TabEvent::TabOpened {
                id,
                waiter_id,
                table,
            } => {
                self.apply_open_tab(id, waiter_id, table);
            }
            TabEvent::FoodOrderPlaced { id, menu_item } => self.apply_order_food(id, menu_item),
            TabEvent::DrinkOrderPlaced { id, menu_item } => self.apply_order_drink(id, menu_item),
            TabEvent::DrinkServed { id, menu_number } => self.apply_drinks_served(id, menu_number),
            TabEvent::FoodPrepared { id, menu_number } => self.apply_food_prepared(id, menu_number),
            TabEvent::FoodServed { id, menu_number } => self.apply_food_served(id, menu_number),
            TabEvent::TabClosed {
                id: _,
                amount_paid: _,
                order_value: _,
                tip_value: _,
            } => todo!(),
        }
    }
}

impl Tab {
    fn apply_drinks_served(&mut self, _id: TabId, menu_number: usize) {
        if let Some(qty) = self.drinks_served.get_mut(&menu_number) {
            *qty += 1;
        } else {
            self.drinks_served.insert(menu_number, 1);
        }
    }

    fn apply_food_prepared(&mut self, _id: TabId, menu_number: usize) {
        if let Some(qty) = self.foods_prepared.get_mut(&menu_number) {
            *qty += 1;
        } else {
            self.foods_prepared.insert(menu_number, 1);
        }
    }

    fn apply_food_served(&mut self, _id: TabId, menu_number: usize) {
        if let Some(qty) = self.foods_served.get_mut(&menu_number) {
            *qty += 1;
        } else {
            self.foods_served.insert(menu_number, 1);
        }
    }

    fn apply_open_tab(&mut self, id: TabId, waiter_id: WaiterId, table: usize) {
        self.id = id;
        self.waiter_id = waiter_id;
        self.table = table;
        self.drink_items = Vec::new();
        self.food_items = Vec::new();
        self.opened = true;
    }

    fn apply_order_drink(&mut self, _id: TabId, menu_item: MenuItem) {
        self.drink_items.push(menu_item);
    }

    fn apply_order_food(&mut self, _id: TabId, menu_item: MenuItem) {
        let mut found = false;
        for food_item in self.food_items.iter_mut() {
            if food_item.menu_number == menu_item.menu_number {
                food_item.quantity += 1;
                found = true;
                break;
            }
        }
        if !found {
            self.food_items.push(menu_item);
        }
    }

    fn drink_fully_served(&self, menu_number: &usize) -> bool {
        let mut ordered_qty = 0;
        for order in self.drink_items.iter() {
            if order.menu_number == *menu_number {
                ordered_qty += order.quantity;
            }
        }
        if ordered_qty > 0 {
            if let Some(served_qty) = self.drinks_served.get(menu_number) {
                if *served_qty == ordered_qty {
                    return true;
                }
            }
        }

        false
    }

    fn food_fully_prepared(&self, menu_number: &usize) -> bool {
        let mut ordered_qty = 0;
        for order in self.food_items.iter() {
            if order.menu_number == *menu_number {
                ordered_qty += order.quantity;
            }
        }
        if ordered_qty > 0 {
            if let Some(prepared_qty) = self.foods_prepared.get(menu_number) {
                if *prepared_qty == ordered_qty {
                    return true;
                }
            }
        }

        false
    }

    fn food_fully_served(&self, menu_number: &usize) -> bool {
        let mut ordered_qty = 0;
        for order in self.food_items.iter() {
            if order.menu_number == *menu_number {
                ordered_qty += order.quantity;
            }
        }
        if ordered_qty > 0 {
            if let Some(served_qty) = self.foods_served.get(menu_number) {
                if *served_qty == ordered_qty {
                    return true;
                }
            }
        }

        false
    }

    fn food_prepared_not_served(&self, menu_number: &usize) -> usize {
        let mut prepared_qty = 0;
        for (prepared_menu_number, qty) in self.foods_prepared.iter() {
            if *prepared_menu_number == *menu_number {
                prepared_qty += qty;
            }
        }
        let mut served_qty = 0;
        for (served_menu_number, qty) in self.foods_served.iter() {
            if *served_menu_number == *menu_number {
                served_qty += qty;
            }
        }
        let result = prepared_qty - served_qty;

        result.max(0)
    }

    fn handle_close_tab_command(
        &self,
        _id: TabId,
        amount_paid: Decimal,
    ) -> Result<Vec<TabEvent>, TabError> {
        let mut subtotal = Decimal::ZERO;
        for food in self.food_items.iter() {
            subtotal += food.price * Decimal::from(food.quantity)
        }
        for drink in self.drink_items.iter() {
            subtotal += drink.price * Decimal::from(drink.quantity)
        }
        let difference = amount_paid - subtotal;
        if difference < Decimal::ZERO {
            return Err(TabError::MustPayEnough);
        }
        let event = TabEvent::TabClosed {
            id: self.id,
            amount_paid,
            order_value: subtotal,
            tip_value: difference,
        };

        Ok(vec![event])
    }

    fn handle_mark_food_prepared_command(
        &self,
        _id: TabId,
        menu_numbers: &[usize],
    ) -> Result<Vec<TabEvent>, TabError> {
        let mut result = Vec::new();
        for menu_number in menu_numbers.iter() {
            let menu_numbers_ordered: Vec<usize> =
                self.food_items.iter().map(|i| i.menu_number).collect();
            if !menu_numbers_ordered.contains(menu_number) || self.food_fully_prepared(menu_number)
            {
                return Err(TabError::FoodNotOutstanding {
                    menu_number: *menu_number,
                });
            }
            result.push(TabEvent::FoodPrepared {
                id: self.id,
                menu_number: *menu_number,
            });
        }

        Ok(result)
    }

    fn handle_mark_food_served_command(
        &self,
        _id: TabId,
        menu_numbers: &[usize],
    ) -> Result<Vec<TabEvent>, TabError> {
        let mut result = Vec::new();
        for menu_number in menu_numbers.iter() {
            let menu_numbers_ordered: Vec<usize> =
                self.food_items.iter().map(|i| i.menu_number).collect();
            if !menu_numbers_ordered.contains(menu_number) || self.food_fully_served(menu_number) {
                return Err(TabError::FoodNotOutstanding {
                    menu_number: *menu_number,
                });
            } else if self.food_prepared_not_served(menu_number) == 0 {
                return Err(TabError::FoodNotPrepared {
                    menu_number: *menu_number,
                });
            }
            result.push(TabEvent::FoodServed {
                id: self.id,
                menu_number: *menu_number,
            });
        }

        Ok(result)
    }

    fn handle_mark_drink_served_command(
        &self,
        _id: TabId,
        menu_numbers: Vec<usize>,
    ) -> Result<Vec<TabEvent>, TabError> {
        let mut result = Vec::new();
        for menu_number in menu_numbers {
            let menu_numbers_ordered: Vec<usize> =
                self.drink_items.iter().map(|i| i.menu_number).collect();
            if !menu_numbers_ordered.contains(&menu_number) || self.drink_fully_served(&menu_number)
            {
                return Err(TabError::DrinkNotOutstanding { menu_number });
            }
            result.push(TabEvent::DrinkServed {
                id: self.id,
                menu_number,
            });
        }

        Ok(result)
    }

    fn handle_place_order_command(
        &self,
        order_items: &[OrderItem],
    ) -> Result<Vec<TabEvent>, TabError> {
        let mut orders = Vec::new();
        for order_item in order_items.iter() {
            let menu_item = MenuItem {
                menu_number: order_item.menu_number,
                description: order_item.description.to_owned(),
                price: order_item.price,
                quantity: 1,
            };
            if order_item.is_drink {
                orders.push(TabEvent::DrinkOrderPlaced {
                    id: self.id,
                    menu_item,
                });
            } else {
                orders.push(TabEvent::FoodOrderPlaced {
                    id: self.id,
                    menu_item,
                });
            }
        }

        Ok(orders)
    }

    fn handle_open_tab_command(
        &self,
        waiter_id: &WaiterId,
        table: usize,
    ) -> Result<Vec<TabEvent>, TabError> {
        Ok(vec![TabEvent::TabOpened {
            id: TabId::new(),
            waiter_id: *waiter_id,
            table,
        }])
    }

    fn tab_is_open(&self) -> bool {
        self.opened
    }

    fn tab_is_open_or_error(&self) -> Result<(), TabError> {
        if !self.tab_is_open() {
            return Err(TabError::TabNotOpened);
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use cqrs_es::test::{AggregateTestExecutor, TestFramework};
    use rust_decimal::Decimal;

    use crate::domain::tab::{
        aggregate::Tab,
        command::{OrderItem, TabCommand},
        error::TabError,
        event::{MenuItem, TabEvent},
        services::TabServices,
        tab_id::TabId,
        waiter_id::WaiterId,
    };

    #[test]
    #[allow(non_snake_case)]
    fn given_unopened_tab_when_PlaceOrder_command_then_TabNotOpened_error() {
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
    fn given_unopened_tab_when_MarkDrinksServed_command_then_TabNotOpened_error() {
        // Arrange
        let tab_services = TabServices {};
        let executor = TestFramework::<Tab>::with(tab_services).given_no_previous_events();

        // Act
        let result = executor
            .when(TabCommand::MarkDrinksServed {
                id: TabId::new(),
                menu_numbers: vec![2],
            })
            .inspect_result();

        // Assert
        assert_eq!(result.err().unwrap(), TabError::TabNotOpened)
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_tab_with_no_events_when_OpenTab_command_then_TabOpened_event() {
        // Arrange
        let expected_waiter_id = WaiterId::new();
        let tab_services = TabServices {};
        let executor = TestFramework::<Tab>::with(tab_services).given_no_previous_events();

        // Act
        let result = executor.when(TabCommand::OpenTab {
            waiter_id: expected_waiter_id,
            table: 1,
        });
        let mut event = result
            .inspect_result()
            .expect("failed to execute command: OpenTab");

        // Assert
        if let Some((tid, wid, table_num)) = match event.pop().unwrap() {
            TabEvent::TabOpened {
                id,
                waiter_id,
                table,
            } => Some((id, waiter_id, table)),
            _ => None,
        } {
            assert!(tid != TabId::default());
            assert_eq!(wid, expected_waiter_id);
            assert_eq!(table_num, 1);
        } else {
            assert!(false, "TabOpened event")
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_opened_tab_when_order_food_then_ItemOrdered_event() {
        // Arrange
        let tab_id = TabId::new();
        let order_items = vec![OrderItem {
            menu_number: 1,
            description: "Steak".into(),
            is_drink: false,
            price: Decimal::from(10),
        }];
        let executor = arrange_executor(tab_id, Some(Vec::new()));

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
                id: tab_id,
                menu_item: MenuItem {
                    menu_number: 1,
                    description: "Steak".into(),
                    price: Decimal::from(10),
                    quantity: 1,
                }
            },
            "ItemOrdered"
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_opened_tab_when_order_1_drink_then_ItemOrdered_event() {
        // Arrange
        let tab_id = TabId::new();
        let order_items = vec![OrderItem {
            menu_number: 2,
            description: "Coca-Cola".into(),
            is_drink: true,
            price: Decimal::from(3),
        }];
        let executor = arrange_executor(tab_id, Some(Vec::new()));

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
                id: tab_id,
                menu_item: MenuItem {
                    menu_number: 2,
                    description: "Coca-Cola".into(),
                    price: Decimal::from(3),
                    quantity: 1,
                }
            },
            "DrinkOrderPlaced"
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_when_order_multiple_items_then_multiple_OrderPlaced_events() {
        // Arrange
        let tab_id = TabId::new();
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
        let executor = arrange_executor(tab_id, Some(Vec::new()));

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
                id: tab_id,
                menu_item: MenuItem {
                    menu_number: 1,
                    description: "Steak".into(),
                    price: Decimal::from(10),
                    quantity: 1,
                }
            },
            "FoodOrderPlaced"
        );
        assert_eq!(
            event[1],
            TabEvent::DrinkOrderPlaced {
                id: tab_id,
                menu_item: MenuItem {
                    menu_number: 2,
                    description: "Coca-Cola".into(),
                    price: Decimal::from(3),
                    quantity: 1,
                }
            },
            "DrinkOrderPlaced"
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_and_drinks_ordered_when_MarkDrinksServed_command_then_DrinkServed_event() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![TabEvent::DrinkOrderPlaced {
                id: tab_id,
                menu_item: MenuItem {
                    menu_number: 2,
                    description: "Coca-Cola".into(),
                    price: Decimal::from(3),
                    quantity: 1,
                },
            }]),
        );

        // Act
        let event = executor
            .when(TabCommand::MarkDrinksServed {
                id: tab_id,
                menu_numbers: vec![2],
            })
            .inspect_result()
            .expect("command MarkDrinkServed failed");

        // Assert
        assert_eq!(event.len(), 1);
        assert_eq!(
            event[0],
            TabEvent::DrinkServed {
                id: tab_id,
                menu_number: 2
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_and_drinks_ordered_when_MarkDrinksServed_command_uses_wrong_menu_number_then_error(
    ) {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![TabEvent::DrinkOrderPlaced {
                id: tab_id,
                menu_item: MenuItem {
                    menu_number: 2,
                    description: "Coca-Cola".into(),
                    price: Decimal::from(3),
                    quantity: 1,
                },
            }]),
        );

        // Act
        let result = executor
            .when(TabCommand::MarkDrinksServed {
                id: tab_id,
                menu_numbers: vec![12],
            })
            .inspect_result();

        // Assert
        assert_eq!(
            result.err().unwrap(),
            TabError::DrinkNotOutstanding { menu_number: 12 }
        )
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_when_MarkDrinksServed_twice_on_same_drink_then_DrinksNotOutstanding_error() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![
                TabEvent::DrinkOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 2,
                        description: "Coca-Cola".into(),
                        price: Decimal::from(3),
                        quantity: 1,
                    },
                },
                TabEvent::DrinkServed {
                    id: tab_id,
                    menu_number: 2,
                },
            ]),
        );

        // Act
        let result = executor
            .when(TabCommand::MarkDrinksServed {
                id: tab_id,
                menu_numbers: vec![2],
            })
            .inspect_result();

        // Assert
        assert_eq!(
            result.err().unwrap(),
            TabError::DrinkNotOutstanding { menu_number: 2 }
        )
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_and_drink_is_ordered_twice_when_MarkDrinksServed_twice_then_DrinkServed_event_twice(
    ) {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![
                TabEvent::DrinkOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 2,
                        description: "Coca-Cola".into(),
                        price: Decimal::from(3),
                        quantity: 1,
                    },
                },
                TabEvent::DrinkOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 2,
                        description: "Coca-Cola".into(),
                        price: Decimal::from(3),
                        quantity: 1,
                    },
                },
                TabEvent::DrinkServed {
                    id: tab_id,
                    menu_number: 2,
                },
            ]),
        );

        // Act
        let event = executor
            .when(TabCommand::MarkDrinksServed {
                id: tab_id,
                menu_numbers: vec![2],
            })
            .inspect_result()
            .expect("command MarkDrinkServed failed");

        // Assert
        assert_eq!(event.len(), 1);
        assert_eq!(
            event[0],
            TabEvent::DrinkServed {
                id: tab_id,
                menu_number: 2
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_when_OpenTab_command_then_error() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(tab_id, Some(Vec::new()));

        // Act
        let result = executor.when(TabCommand::OpenTab {
            waiter_id: WaiterId::new(),
            table: 1,
        });

        // Assert
        result.then_expect_error(TabError::TabIsOpen { id: tab_id })
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_when_MarkFoodPrepared_command_then_FoodNotOutstanding_error() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(tab_id, Some(Vec::new()));

        // Act
        let result = executor.when(TabCommand::MarkFoodPrepared {
            id: tab_id,
            menu_numbers: vec![1],
        });

        // Assert
        result.then_expect_error(TabError::FoodNotOutstanding { menu_number: 1 })
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_no_tab_when_MarkFoodPrepared_command_then_TabNotOpen_error() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(tab_id, None);

        // Act
        let result = executor.when(TabCommand::MarkFoodPrepared {
            id: tab_id,
            menu_numbers: vec![1],
        });

        // Assert
        result.then_expect_error(TabError::TabNotOpened);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_and_food_ordered_when_MarkFoodPrepared_command_then_FoodPrepared_event() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![TabEvent::FoodOrderPlaced {
                id: tab_id,
                menu_item: MenuItem {
                    menu_number: 1,
                    description: "Steak".into(),
                    price: Decimal::from(10),
                    quantity: 1,
                },
            }]),
        );

        // Act
        let event = executor
            .when(TabCommand::MarkFoodPrepared {
                id: tab_id,
                menu_numbers: vec![1],
            })
            .inspect_result()
            .expect("command MarkFoodPrepared failed");

        // Assert
        assert_eq!(event.len(), 1);
        assert_eq!(
            event[0],
            TabEvent::FoodPrepared {
                id: tab_id,
                menu_number: 1
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_when_MarkFoodPrepared_twice_on_same_food_then_FoodNotOutstanding_error() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![
                TabEvent::FoodOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 1,
                        description: "Steak".into(),
                        price: Decimal::from(10),
                        quantity: 1,
                    },
                },
                TabEvent::FoodPrepared {
                    id: tab_id,
                    menu_number: 1,
                },
            ]),
        );

        // Act
        let result = executor.when(TabCommand::MarkFoodPrepared {
            id: tab_id,
            menu_numbers: vec![1],
        });

        // Assert
        result.then_expect_error(TabError::FoodNotOutstanding { menu_number: 1 });
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_and_food_is_ordered_twice_when_MarkFoodPrepared_twice_then_FoodPrepared_event_twice(
    ) {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![
                TabEvent::FoodOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 1,
                        description: "Steak".into(),
                        price: Decimal::from(10),
                        quantity: 1,
                    },
                },
                TabEvent::FoodOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 1,
                        description: "Steak".into(),
                        price: Decimal::from(10),
                        quantity: 1,
                    },
                },
                TabEvent::FoodPrepared {
                    id: tab_id,
                    menu_number: 1,
                },
            ]),
        );

        // Act
        let event = executor
            .when(TabCommand::MarkFoodPrepared {
                id: tab_id,
                menu_numbers: vec![1],
            })
            .inspect_result()
            .expect("command MarkFoodPrepared failed");

        // Assert
        assert_eq!(event.len(), 1);
        assert_eq!(
            event[0],
            TabEvent::FoodPrepared {
                id: tab_id,
                menu_number: 1
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_no_tab_when_MarkFoodServed_command_then_TabNotOpen_error() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(tab_id, None);

        // Act
        let result = executor.when(TabCommand::MarkFoodServed {
            id: tab_id,
            menu_numbers: vec![1],
        });

        // Assert
        result.then_expect_error(TabError::TabNotOpened);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_when_MarkFoodServed_command_then_FoodNotOutstanding_error() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(tab_id, Some(Vec::new()));

        // Act
        let result = executor.when(TabCommand::MarkFoodServed {
            id: tab_id,
            menu_numbers: vec![1],
        });

        // Assert
        result.then_expect_error(TabError::FoodNotOutstanding { menu_number: 1 })
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_and_not_food_prepared_when_MarkFoodServed_command_then_FoodNotOutstanding_error(
    ) {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![TabEvent::FoodOrderPlaced {
                id: tab_id,
                menu_item: MenuItem {
                    menu_number: 1,
                    description: "Steak".into(),
                    price: Decimal::from(10),
                    quantity: 1,
                },
            }]),
        );

        // Act
        let result = executor.when(TabCommand::MarkFoodServed {
            id: tab_id,
            menu_numbers: vec![1],
        });

        // Assert
        result.then_expect_error(TabError::FoodNotPrepared { menu_number: 1 })
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_and_food_prepared_when_MarkFoodServed_command_then_FoodServed_event() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![
                TabEvent::FoodOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 1,
                        description: "Steak".into(),
                        price: Decimal::from(10),
                        quantity: 1,
                    },
                },
                TabEvent::FoodPrepared {
                    id: tab_id,
                    menu_number: 1,
                },
            ]),
        );

        // Act
        let event = executor
            .when(TabCommand::MarkFoodServed {
                id: tab_id,
                menu_numbers: vec![1],
            })
            .inspect_result()
            .expect("command MarkFoodServed failed");

        // Assert
        assert_eq!(event.len(), 1);
        assert_eq!(
            event[0],
            TabEvent::FoodServed {
                id: tab_id,
                menu_number: 1
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_when_MarkFoodServed_twice_on_same_food_then_FoodNotOutstanding_error() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![
                TabEvent::FoodOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 1,
                        description: "Steak".into(),
                        price: Decimal::from(10),
                        quantity: 1,
                    },
                },
                TabEvent::FoodPrepared {
                    id: tab_id,
                    menu_number: 1,
                },
                TabEvent::FoodServed {
                    id: tab_id,
                    menu_number: 1,
                },
            ]),
        );

        // Act
        let result = executor.when(TabCommand::MarkFoodServed {
            id: tab_id,
            menu_numbers: vec![1],
        });

        // Assert
        result.then_expect_error(TabError::FoodNotOutstanding { menu_number: 1 });
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_and_food_is_ordered_twice_but_served_once_when_MarkFoodServed_then_FoodServed_event(
    ) {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![
                TabEvent::FoodOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 1,
                        description: "Steak".into(),
                        price: Decimal::from(10),
                        quantity: 1,
                    },
                },
                TabEvent::FoodOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 1,
                        description: "Steak".into(),
                        price: Decimal::from(10),
                        quantity: 1,
                    },
                },
                TabEvent::FoodPrepared {
                    id: tab_id,
                    menu_number: 1,
                },
                TabEvent::FoodServed {
                    id: tab_id,
                    menu_number: 1,
                },
                TabEvent::FoodPrepared {
                    id: tab_id,
                    menu_number: 1,
                },
            ]),
        );

        // Act
        let event = executor
            .when(TabCommand::MarkFoodServed {
                id: tab_id,
                menu_numbers: vec![1],
            })
            .inspect_result()
            .expect("command MarkFoodServed failed");

        // Assert
        assert_eq!(event.len(), 1);
        assert_eq!(
            event[0],
            TabEvent::FoodServed {
                id: tab_id,
                menu_number: 1
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_unopen_tab_when_CloseTab_command_then_TabNotOpen_error() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(tab_id, None);

        // Act
        let result = executor.when(TabCommand::CloseTab {
            id: tab_id,
            amount_paid: Decimal::from(16),
        });

        // Assert
        result.then_expect_error(TabError::TabNotOpened);
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_when_CloseTab_command_with_extra_amount_then_TabClosed_event_with_tip() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![
                TabEvent::FoodOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 1,
                        description: "Steak".into(),
                        price: Decimal::from(10),
                        quantity: 1,
                    },
                },
                TabEvent::DrinkOrderPlaced {
                    id: tab_id,
                    menu_item: MenuItem {
                        menu_number: 2,
                        description: "Coca-Cola".into(),
                        price: Decimal::from(5),
                        quantity: 1,
                    },
                },
                TabEvent::FoodPrepared {
                    id: tab_id,
                    menu_number: 1,
                },
                TabEvent::FoodServed {
                    id: tab_id,
                    menu_number: 1,
                },
                TabEvent::DrinkServed {
                    id: tab_id,
                    menu_number: 2,
                },
            ]),
        );

        // Act
        let event = executor
            .when(TabCommand::CloseTab {
                id: tab_id,
                amount_paid: Decimal::from(16),
            })
            .inspect_result()
            .expect("command MarkFoodServed failed");

        // Assert
        assert_eq!(event.len(), 1);
        assert_eq!(
            event[0],
            TabEvent::TabClosed {
                id: tab_id,
                amount_paid: Decimal::from(16),
                order_value: Decimal::from(15),
                tip_value: Decimal::from(1),
            }
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_open_tab_when_CloseTab_amount_paid_not_enough_then_MustPayEnough_error() {
        // Arrange
        let tab_id = TabId::new();
        let executor = arrange_executor(
            tab_id,
            Some(vec![TabEvent::DrinkOrderPlaced {
                id: tab_id,
                menu_item: MenuItem {
                    menu_number: 2,
                    description: "Coca-Cola".into(),
                    price: Decimal::from(5),
                    quantity: 1,
                },
            }]),
        );

        // Act
        let result = executor.when(TabCommand::CloseTab {
            id: tab_id,
            amount_paid: Decimal::from_str("4.99").unwrap(),
        });

        // Assert
        result.then_expect_error(TabError::MustPayEnough);
    }

    fn arrange_executor(
        tab_id: TabId,
        given_events: Option<Vec<TabEvent>>,
    ) -> AggregateTestExecutor<Tab> {
        let waiter_id = WaiterId::new();
        let tab_services = TabServices {};

        match given_events {
            Some(mut events) => {
                let mut all_events = Vec::new();
                all_events.push(TabEvent::TabOpened {
                    id: tab_id,
                    waiter_id,
                    table: 1,
                });
                all_events.append(&mut events);
                TestFramework::<Tab>::with(tab_services).given(all_events)
            }
            None => TestFramework::<Tab>::with(tab_services).given_no_previous_events(),
        }
    }
}
