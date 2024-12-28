use serde::{ser::SerializeStruct, Serialize};
use crate::component::{ComponentTrait, ObjectId, NodeId};
use serde_inner_serialize::{InnerSerializableTrait, OuterSerializable, OuterSerializableTrait};

pub trait DiscoverableEntity : ComponentTrait + ObjectId + NodeId {
    fn config_topic(&self) -> String;
}

impl <T> DiscoverableEntity for T where T: ComponentTrait + ObjectId + NodeId {
    fn config_topic(&self) -> String {
        let oid = self.object_id().expect(crate::component::OBJECT_ID_OR_NAME_NOT_SET).to_lowercase();
        if let Some(node_id) = self.node_id() {
            format!(
                "{}/{}/{}/{}/config",
                self.component().prefix().to_lowercase(),
                self.component(),
                node_id.to_lowercase(),
                oid
            )
        } else {
            format!(
                "{}/{}/{}/config",
                self.component().prefix().to_lowercase(),
                self.component(),
                oid
            )
        }
    }
}


#[derive(OuterSerializable)]
pub struct Discoverable<'a, T: DiscoverableEntity + InnerSerializableTrait> {
    inner: &'a T,
}

impl<'a, T> Discoverable<'a, T> where T: ComponentTrait + ObjectId + NodeId + Serialize + InnerSerializableTrait
{
    pub fn new(inner: &'a T) -> Self {
        Self { inner }
    }
}

impl <'a, T> Serialize for Discoverable<'a, T> where T: ComponentTrait + ObjectId + NodeId + Serialize + InnerSerializableTrait {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        
        if self.inner.object_id().is_none() {
            return Err(serde::ser::Error::custom(crate::component::OBJECT_ID_OR_NAME_NOT_SET));
        }

        let fields = T::count_fields();
        let mut state = serializer.serialize_struct(&self.get_full_type_name(), fields+3)?;
        state.serialize_field("config_topic", &self.inner.config_topic())?;
        state.serialize_field("platform", &self.inner.component())?;
        state.serialize_field("prefix", &self.inner.component().prefix())?;
        self.inner.inner_serialize(&mut state).unwrap();
        state.end()
    }
    
}


#[cfg(test)]
mod tests {
    use crate::{component::DefaultStateTopic, components::{button::Button, select::Select}};

    use super::*;

    #[test]
    fn test_discoverable() {
        let button = Button::new("command_topic".to_string())
            .with_object_id(uuid::Uuid::new_v4().to_string());
        let mut select = Select::new("command_topic2".to_string(), vec!["option1".to_string(), "option2".to_string()])
            .with_object_id(uuid::Uuid::new_v4().to_string());

        let state_topic = select.default_state_topic();
        select = select.with_state_topic(state_topic);

        let discoverable_button = Discoverable::new(&button);
        let discoverable_select = Discoverable::new(&select);
        let serialized_button = serde_json::to_string(&discoverable_button).unwrap();
        println!("\n\nBUTTON:\n{}\n", serialized_button);
        let serialized_select = serde_json::to_string(&discoverable_select).unwrap();
        println!("\n\nSELECT:\n{}\n", serialized_select);
        
    }
}


