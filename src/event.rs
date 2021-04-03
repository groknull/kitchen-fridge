//! Calendar events

use serde::{Deserialize, Serialize};

use crate::item::ItemId;
use crate::item::SyncStatus;
use crate::item::VersionTag;

/// TODO: implement Event one day.
/// This crate currently only supports tasks, not calendar events.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    id: ItemId,
    name: String,
    sync_status: SyncStatus,
}

impl Event {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn id(&self) -> &ItemId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sync_status(&self) -> &SyncStatus {
        &self.sync_status
    }
    pub fn set_sync_status(&mut self, new_status: SyncStatus) {
        self.sync_status = new_status;
    }
}
