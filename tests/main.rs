use cafe_tab::{
    domain::tab::{
        command::{OrderItem, TabCommand},
        services::TabServices,
        tab_id::TabId,
        waiter_id::WaiterId,
    },
    infrasctructure::{
        persistence::context::{
            connection_parameters::ConnectionBuilder,
            postgres::{create_db, migrate_db, postgres_pool},
        },
        respository::postgresql::cqrs::cqrs_tab,
    },
    shared_kernel::KitchenTabViewRepository,
};
use rust_decimal::Decimal;
use secrecy::Secret;
use uuid::Uuid;

#[tokio::test]
async fn initially_kitchen_todo_list_is_empty() {
    // Arrange
    let db_name = Uuid::new_v4().to_string();
    let params = ConnectionBuilder::new()
        .with_credentials("postgres", Secret::new(String::from("password")))
        .with_database(&db_name)
        .build();
    create_db(&params).await;
    migrate_db(&params).await;
    let pool = postgres_pool(&params).await;
    let kitchen_tab_view_repo = KitchenTabViewRepository::new(pool.clone());
    let services = TabServices {};
    let tab_aggregate = cqrs_tab(pool, services, kitchen_tab_view_repo.clone());

    // Act
    let id = TabId::default();
    let id_string = id.to_string();
    tab_aggregate
        .execute(
            &id_string,
            TabCommand::OpenTab {
                id,
                waiter_id: WaiterId::new(),
                table: 1,
            },
        )
        .await
        .expect("failed to open tab");

    // Assert
    let actual = kitchen_tab_view_repo
        .load(&id_string)
        .await
        .expect("failed to load the kitchen tab view")
        .unwrap();
    assert_eq!(actual.len(), 0);
}

#[tokio::test]
async fn given_new_tab_when_1_food_order_then_kitchen_list_view_shows_1_food_order() {
    // Arrange
    let db_name = Uuid::new_v4().to_string();
    let params = ConnectionBuilder::new()
        .with_credentials("postgres", Secret::new(String::from("password")))
        .with_database(&db_name)
        .build();
    create_db(&params).await;
    migrate_db(&params).await;
    let pool = postgres_pool(&params).await;
    let kitchen_tab_view_repo = KitchenTabViewRepository::new(pool.clone());
    let services = TabServices {};
    let tab_aggregate = cqrs_tab(pool, services, kitchen_tab_view_repo.clone());
    let id = TabId::default();
    let id_string = id.to_string();
    tab_aggregate
        .execute(
            &id_string,
            TabCommand::OpenTab {
                id,
                waiter_id: WaiterId::new(),
                table: 1,
            },
        )
        .await
        .expect("failed to open tab");

    // Act
    tab_aggregate
        .execute(
            &id_string,
            TabCommand::PlaceOrder {
                order_items: vec![OrderItem {
                    menu_number: 1,
                    description: "Steak".into(),
                    is_drink: false,
                    price: Decimal::from(10),
                }],
            },
        )
        .await
        .expect("failed to order food");

    // Assert
    let actual = kitchen_tab_view_repo
        .load(&id_string)
        .await
        .expect("failed to load the kitchen tab view")
        .unwrap();
    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0].tab_id(), id);
    assert_eq!(actual[0].food_items().len(), 1);
    assert_eq!(actual[0].food_items()[0].menu_number(), 1);
    assert_eq!(actual[0].food_items()[0].description(), "Steak");
}
