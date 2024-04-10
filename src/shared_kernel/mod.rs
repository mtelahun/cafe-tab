use std::ops::Deref;
use std::sync::Arc;

use cqrs_es::persist::GenericQuery;
use cqrs_es::persist::PersistenceError;
use cqrs_es::persist::ViewRepository;
use postgres_es::PostgresViewRepository;
use sqlx::{Pool, Postgres};

use crate::domain::tab::aggregate::Tab;
use crate::domain::tab::queries::kitchen::KitchenTodoList;
use crate::domain::tab::queries::open_tabs::WaiterTodoList;

pub type KitchenTabQuery =
    GenericQuery<PostgresViewRepository<KitchenTodoList, Tab>, KitchenTodoList, Tab>;

#[derive(Clone)]
pub struct KitchenTabViewRepository(Arc<PostgresViewRepository<KitchenTodoList, Tab>>);

pub type WaiterTabQuery =
    GenericQuery<PostgresViewRepository<WaiterTodoList, Tab>, WaiterTodoList, Tab>;

#[derive(Clone)]
pub struct WaiterTabViewRepository(Arc<PostgresViewRepository<WaiterTodoList, Tab>>);

impl KitchenTabViewRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self(Arc::new(PostgresViewRepository::new(
            "kitchen_tab_query",
            pool.clone(),
        )))
    }

    pub async fn load(&self, view_id: &str) -> Result<Option<KitchenTodoList>, PersistenceError> {
        self.0.load(view_id).await
    }
}

impl WaiterTabViewRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self(Arc::new(PostgresViewRepository::new(
            "waiter_tab_query",
            pool.clone(),
        )))
    }

    pub async fn load(&self, view_id: &str) -> Result<Option<WaiterTodoList>, PersistenceError> {
        self.0.load(view_id).await
    }
}

impl std::ops::Deref for KitchenTabViewRepository {
    type Target = Arc<PostgresViewRepository<KitchenTodoList, Tab>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for WaiterTabViewRepository {
    type Target = Arc<PostgresViewRepository<WaiterTodoList, Tab>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<KitchenTabViewRepository> for Arc<PostgresViewRepository<KitchenTodoList, Tab>> {
    fn from(value: KitchenTabViewRepository) -> Self {
        value.deref().clone()
    }
}

impl From<WaiterTabViewRepository> for Arc<PostgresViewRepository<WaiterTodoList, Tab>> {
    fn from(value: WaiterTabViewRepository) -> Self {
        value.deref().clone()
    }
}
