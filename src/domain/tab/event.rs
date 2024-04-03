use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TabEvent {}

impl DomainEvent for TabEvent {
    fn event_type(&self) -> String {
        todo!()
    }

    fn event_version(&self) -> String {
        todo!()
    }
}
