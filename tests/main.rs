use cafe_tab::{
    domain::tab::{command::TabCommand, services::TabServices, tab_id::TabId, waiter_id::WaiterId},
    infrasctructure::{
        persistence::context::{
            connection_parameters::ConnectionBuilder,
            postgres::{create_db, migrate_db, postgres_pool},
        },
        respository::postgresql::cqrs::cqrs_tab,
    },
    shared_kernel::KitchenTabViewRepository,
};
use secrecy::Secret;
use uuid::Uuid;

#[tokio::test]
async fn initially_tab_is_empty() {
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
    assert_eq!(actual.tab_id(), id);
    assert_eq!(actual.food_items().len(), 0);
}
