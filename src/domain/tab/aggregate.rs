use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    command::TabCommand, error::TabError, event::TabEvent, services::TabServices, tab_id::TabId,
    waiter_id::WaiterId,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tab {
    id: TabId,
    table: usize,
    waiter_id: WaiterId,
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
                return Ok(vec![TabEvent::TabOpened { waiter_id, table }])
            }
            TabCommand::OrderItem => return Err(TabError::TabNotOpened),
        };
    }

    fn apply(&mut self, _event: Self::Event) {
        todo!()
    }
}

impl Tab {
    pub fn new(waiter_id: WaiterId, table: usize) -> Self {
        Tab {
            id: TabId::new(),
            table,
            waiter_id,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use cqrs_es::test::TestFramework;

    use crate::domain::tab::{
        aggregate::Tab, command::TabCommand, error::TabError, event::TabEvent,
        services::TabServices, waiter_id::WaiterId,
    };

    #[test]
    #[allow(non_snake_case)]
    fn given_unopened_tab_when_any_command_then_TabNotOpened_error() {
        // Arrange
        let tab_services = TabServices {};
        let executor = TestFramework::<Tab>::with(tab_services).given_no_previous_events();

        // Act
        let result = executor.when(TabCommand::OrderItem).inspect_result();

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
}
