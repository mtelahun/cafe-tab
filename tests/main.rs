use cafe_tab::domain::tab::command::{OrderItem, TabCommand};
use rust_decimal::Decimal;

use crate::test_state::{AggregateState, TestState};

pub mod test_state;

#[tokio::test]
async fn initially_kitchen_todo_list_is_empty() {
    // Act
    let state = TestState::new(AggregateState::Open).await;

    // Assert
    let actual = state.load_kitchen_todo_list().await;
    assert_eq!(actual.len(), 0);
}

#[tokio::test]
async fn given_new_tab_when_1_food_order_then_kitchen_list_view_shows_1_food_order() {
    // Arrange
    let state = TestState::new(AggregateState::Open).await;

    // Act
    state
        .execute_command(TabCommand::PlaceOrder {
            order_items: vec![OrderItem {
                menu_number: 1,
                description: "Steak".into(),
                is_drink: false,
                price: Decimal::from(10),
            }],
        })
        .await;

    // Assert
    let actual = state.load_kitchen_todo_list().await;
    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0].tab_id(), state.tab_id);
    assert_eq!(actual[0].food_items().len(), 1);
    assert_eq!(actual[0].food_items()[0].menu_number(), 1);
    assert_eq!(actual[0].food_items()[0].description(), "Steak");
}

#[tokio::test]
async fn given_tab_with_1_food_order_when_another_food_order_then_kitchen_list_view_shows_2_food_orders(
) {
    // Arrange
    let state = TestState::new(AggregateState::Open).await;
    state
        .execute_command(TabCommand::PlaceOrder {
            order_items: vec![OrderItem {
                menu_number: 1,
                description: "Steak".into(),
                is_drink: false,
                price: Decimal::from(10),
            }],
        })
        .await;

    // Act
    state
        .execute_command(TabCommand::PlaceOrder {
            order_items: vec![OrderItem {
                menu_number: 1,
                description: "Steak".into(),
                is_drink: false,
                price: Decimal::from(10),
            }],
        })
        .await;

    // Assert
    let actual = state.load_kitchen_todo_list().await;
    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0].tab_id(), state.tab_id);
    assert_eq!(actual[0].food_items().len(), 2);
    assert_eq!(actual[0].food_items()[0].menu_number(), 1);
    assert_eq!(actual[0].food_items()[0].description(), "Steak");
    assert_eq!(actual[0].food_items()[1].menu_number(), 1);
    assert_eq!(actual[0].food_items()[1].description(), "Steak");
}

#[tokio::test]
async fn given_tab_with_1_food_order_when_kitchen_marks_it_prepared_then_kitchen_todo_list_shows_0_food_orders(
) {
    // Arrange
    let state = TestState::new(AggregateState::Open).await;
    state
        .execute_command(TabCommand::PlaceOrder {
            order_items: vec![OrderItem {
                menu_number: 1,
                description: "Steak".into(),
                is_drink: false,
                price: Decimal::from(10),
            }],
        })
        .await;

    // Act
    state
        .execute_command(TabCommand::MarkFoodPrepared {
            id: state.tab_id,
            menu_numbers: vec![1],
        })
        .await;

    // Assert
    let actual = state.load_kitchen_todo_list().await;
    assert_eq!(actual.len(), 0);
}

#[tokio::test]
async fn initially_waiter_todo_list_is_empty() {
    // Act
    let state = TestState::new(AggregateState::Open).await;

    // Assert
    let actual = state.get_waiter_todo_list().await;
    assert_eq!(actual.len(), 0);
}

#[tokio::test]
async fn given_new_tab_when_1_food_order_then_waiter_list_view_shows_1_food_order() {
    // Arrange
    let state = TestState::new(AggregateState::Open).await;

    // Act
    state
        .execute_command(TabCommand::PlaceOrder {
            order_items: vec![OrderItem {
                menu_number: 1,
                description: "Steak".into(),
                is_drink: false,
                price: Decimal::from(10),
            }],
        })
        .await;

    // Assert
    let actual = state.get_waiter_todo_list().await;
    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0].id(), state.tab_id);
    assert_eq!(actual[0].open_items().len(), 1);
    assert_eq!(actual[0].open_items()[0].menu_number(), 1);
    assert_eq!(actual[0].open_items()[0].description(), "Steak");
}
