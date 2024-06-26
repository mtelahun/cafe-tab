use std::sync::Arc;

use cqrs_es::{persist::PersistedEventStore, CqrsFramework, Query};
use postgres_es::{postgres_cqrs, PostgresEventRepository};
use sqlx::{Pool, Postgres};

use crate::{
    domain::tab::{
        aggregate::Tab, queries::simple_logging::SimpleLoggingQuery, services::TabServices,
    },
    shared_kernel::{
        KitchenTabQuery, KitchenTabViewRepository, WaiterTabQuery, WaiterTabViewRepository,
    },
};

pub type TabCqrsFramework =
    Arc<CqrsFramework<Tab, PersistedEventStore<PostgresEventRepository, Tab>>>;

pub fn cqrs_tab(
    pool: Pool<Postgres>,
    services: TabServices,
    waiter_todo_repo: WaiterTabViewRepository,
    repo: KitchenTabViewRepository,
) -> TabCqrsFramework {
    let logging_query = SimpleLoggingQuery {};
    let mut kitchen_tab_query = KitchenTabQuery::new(repo.into());
    let mut waiter_tab_query = WaiterTabQuery::new(waiter_todo_repo.into());
    kitchen_tab_query.use_error_handler(Box::new(|e| eprintln!("{e}")));
    waiter_tab_query.use_error_handler(Box::new(|e| eprintln!("{e}")));
    let queries: Vec<Box<dyn Query<Tab>>> = vec![
        Box::new(kitchen_tab_query),
        Box::new(waiter_tab_query),
        Box::new(logging_query),
    ];

    Arc::new(postgres_cqrs(pool, queries, services))
}
