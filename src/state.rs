use serde::Serialize;
use crate::component::{ComponentTrait, ObjectId, NodeId};

pub trait State {
    fn state_topic(&self) -> String;
}

impl<T> State for T
where
    T: ComponentTrait + ObjectId + NodeId + Serialize,
{
    fn state_topic(&self) -> String {
        if let Some(node_id) = self.node_id() {
            format!(
                "{}/{}/{}/{}/state",
                self.prefix().to_lowercase(),
                self.component(),
                node_id.to_lowercase(),
                self.object_id().to_lowercase()
            )
        } else {
            format!(
                "{}/{}/{}/state",
                self.prefix().to_lowercase(),
                self.component(),
                self.object_id().to_lowercase()
            )
        }
    }
}
