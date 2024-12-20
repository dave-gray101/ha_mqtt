use serde::{ser::SerializeStruct, Serialize};
use crate::component::{ComponentTrait, ObjectId, NodeId};

#[derive(serde::Serialize)]
pub struct Discoverable<T: serde::Serialize> {
    #[serde(flatten)]
    inner: T,
}

impl<T> Discoverable<T> where T: ComponentTrait + ObjectId + NodeId + Serialize
{

    fn new(inner: T) -> Self {
        Self {
            inner,
        }
    }

    fn discoverable_name(&self) -> String {
        format!("Discoverable<{}>", &self.inner.component().to_string())
    }

    

    fn config_topic(&self) -> String {
        if let Some(node_id) = self.inner.node_id() {
            format!(
                "{}/{}/{}/{}/config",
                self.inner.prefix().to_lowercase(),
                self.inner.component(),
                node_id.to_lowercase(),
                self.inner.object_id().to_lowercase()
            )
        } else {
            format!(
                "{}/{}/{}/config",
                self.inner.prefix().to_lowercase(),
                self.inner.component(),
                self.inner.object_id().to_lowercase()
            )
        }
    }

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
       
        let mut state = serializer.serialize_struct(std::any::type_name_of_val(&self.inner), 4)?;
        state.serialize_field("config_topic", &self.config_topic())?;
        state.serialize_field("platform", &self.inner.component())?;
        state.serialize_field("prefix", &self.prefix())?;
        state.serialize_field("inner", &self.inner)?;
        
        return state.end();
        }

}


#[cfg(test)]
mod tests {
    use crate::components::button::{self, Button};

    use super::*;

    #[test]
    fn test_discoverable() {
        let button = Button::new("command_topic".to_string());
        let discoverable_button = Discoverable::new(button);
        let serialized = serde_json::to_string(&qos).unwrap();
        println!("{}", serialized);
    }
}