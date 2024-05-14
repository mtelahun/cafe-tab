use cafe_tab::{
    domain::tab::{
        command::TabCommand,
        queries::{kitchen::KitchenTodoList, open_tabs::WaiterTodoList},
        services::TabServices,
        tab_id::TabId,
        waiter_id::WaiterId,
    },
    infrasctructure::{
        persistence::context::{
            connection_parameters::ConnectionBuilder,
            postgres::{create_db, migrate_db, postgres_pool},
        },
        respository::postgresql::cqrs::{cqrs_tab, TabCqrsFramework},
    },
    shared_kernel::{KitchenTabViewRepository, WaiterTabViewRepository},
};
use secrecy::Secret;
use uuid::Uuid;

pub struct TestState {
    pub tab_id: TabId,
    pub tab_aggregate: TabCqrsFramework,
    pub tab_kitchen_todo_list: KitchenTabViewRepository,
    pub waiter_todo_list: WaiterTabViewRepository,
}

#[derive(Debug)]
pub enum AggregateState {
    Open,
    None,
}

impl TestState {
    pub async fn new(aggregate_state: AggregateState) -> Self {
        let db_name = Uuid::new_v4().to_string();
        let params = ConnectionBuilder::new()
            .with_credentials("postgres", Secret::new(String::from("password")))
            .with_database(&db_name)
            .build();
        create_db(&params).await;
        migrate_db(&params).await;
        let pool = postgres_pool(&params).await;
        let services = TabServices {};
        let waiter_todo_list = WaiterTabViewRepository::new(pool.clone());
        let tab_kitchen_todo_list = KitchenTabViewRepository::new(pool.clone());
        let tab_aggregate = cqrs_tab(
            pool,
            services,
            waiter_todo_list.clone(),
            tab_kitchen_todo_list.clone(),
        );
        let tab_id = TabId::new();
        let waiter_id = WaiterId::new();
        Self::initialize_aggregate_state(&tab_aggregate, tab_id, waiter_id, aggregate_state).await;

        Self {
            tab_id,
            tab_kitchen_todo_list,
            waiter_todo_list,
            tab_aggregate,
        }
    }

    pub async fn execute_command(&self, command: TabCommand) {
        self.tab_aggregate
            .execute(&self.tab_id.to_string(), command)
            .await
            .expect("failed to order execute a command on the aggregate");
    }

    pub async fn load_kitchen_todo_list(&self) -> KitchenTodoList {
        self.tab_kitchen_todo_list
            .load(&self.tab_id.to_string())
            .await
            .expect("failed to load the kitchen tab view")
            .unwrap()
    }

    pub async fn get_waiter_todo_list(&self) -> WaiterTodoList {
        self.waiter_todo_list
            .load(&self.tab_id.to_string())
            .await
            .expect("failed to load the waiter tab view")
            .unwrap()
    }

    async fn initialize_aggregate_state(
        tab_aggregate: &TabCqrsFramework,
        tab_id: TabId,
        waiter_id: WaiterId,
        aggregate_state: AggregateState,
    ) {
        match aggregate_state {
            AggregateState::Open => {
                tab_aggregate
                    .execute(
                        &tab_id.to_string(),
                        TabCommand::OpenTab {
                            id: tab_id,
                            waiter_id,
                            table: 1,
                        },
                    )
                    .await
                    .expect("failed to open tab");
            }
            AggregateState::None => {}
        }
    }
}
