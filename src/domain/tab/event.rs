use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use super::waiter_id::WaiterId;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TabEvent {
    TabOpened { waiter_id: WaiterId, table: usize },
}

impl DomainEvent for TabEvent {
    fn event_type(&self) -> String {
        todo!()
    }

    fn event_version(&self) -> String {
        todo!()
    }
}
