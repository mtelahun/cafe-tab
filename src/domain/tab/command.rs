use serde::Deserialize;

use super::waiter_id::WaiterId;

#[derive(Debug, Deserialize)]
pub enum TabCommand {
    OpenTab { waiter_id: WaiterId, table: usize },
}
