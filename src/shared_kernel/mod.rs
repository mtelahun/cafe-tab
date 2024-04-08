use std::ops::Deref;
use std::sync::Arc;

use cqrs_es::persist::GenericQuery;
use cqrs_es::persist::PersistenceError;
use cqrs_es::persist::ViewRepository;
use postgres_es::PostgresViewRepository;
use sqlx::{Pool, Postgres};

use crate::domain::tab::{aggregate::Tab, queries::kitchen::KitchenTabView};

pub type KitchenTabQuery =
    GenericQuery<PostgresViewRepository<KitchenTabView, Tab>, KitchenTabView, Tab>;

#[derive(Clone)]
pub struct KitchenTabViewRepository(Arc<PostgresViewRepository<KitchenTabView, Tab>>);

impl KitchenTabViewRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self(Arc::new(PostgresViewRepository::new(
            "kitchen_tab_query",
            pool.clone(),
        )))
    }

    pub async fn load(&self, view_id: &str) -> Result<Option<KitchenTabView>, PersistenceError> {
        self.0.load(view_id).await
    }
}

impl std::ops::Deref for KitchenTabViewRepository {
    type Target = Arc<PostgresViewRepository<KitchenTabView, Tab>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<KitchenTabViewRepository> for Arc<PostgresViewRepository<KitchenTabView, Tab>> {
    fn from(value: KitchenTabViewRepository) -> Self {
        value.deref().clone()
    }
}
