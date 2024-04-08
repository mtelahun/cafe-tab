use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};

use crate::domain::tab::aggregate::Tab;

#[derive(Debug, Default)]
pub struct SimpleLoggingQuery {}

#[async_trait]
impl Query<Tab> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Tab>]) {
        for event in events {
            println!(
                "{aggregate_id:<40} {:>3}\n{:#?}",
                event.sequence, event.payload
            );
        }
    }
}
