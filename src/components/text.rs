use crate::availability::{Availability, AvailabilityMode};
use crate::device::Device;
use crate::qos::Qos;
use serde::Serialize;

use serde_inner_serialize::InnerSerializable;
use serde_inner_serialize::InnerSerializableTrait;

#[derive(Debug, Default, Clone, PartialEq, Serialize, InnerSerializable)]
pub struct Text<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<AvailabilityMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    pub command_topic: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<&'a Device>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<TextEntityCategory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<TextMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TextEntityCategory {
    Config,
    Diagnostic,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TextMode {
    Text,
    Password,
}

impl<'a> Text<'a> {
    pub fn new(command_topic: String) -> Text<'a> {
        Text {
            command_topic,
            ..Text::default()
        }
    }

    pub fn with_availability(mut self, availability: Availability) -> Self {
        self.availability = Some(availability);
        self
    }

    pub fn with_availability_mode(mut self, availability_mode: AvailabilityMode) -> Self {
        self.availability_mode = Some(availability_mode);
        self
    }

    pub fn with_availability_template(mut self, availability_template: impl Into<String>) -> Self {
        self.availability_template = Some(availability_template.into());
        self
    }

    pub fn with_availability_topic(mut self, availability_topic: impl Into<String>) -> Self {
        self.availability_topic = Some(availability_topic.into());
        self
    }

    pub fn with_command_template(mut self, command_template: impl Into<String>) -> Self {
        self.command_template = Some(command_template.into());
        self
    }

    pub fn with_device(mut self, device: &'a Device) -> Self {
        self.device = Some(device);
        self
    }

    pub fn with_entity_category(mut self, entity_category: TextEntityCategory) -> Self {
        self.entity_category = Some(entity_category);
        self
    }

    pub fn with_enabled_by_default(mut self, enabled_by_default: bool) -> Self {
        self.enabled_by_default = Some(enabled_by_default);
        self
    }

    pub fn with_encoding(mut self, encoding: impl Into<String>) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_json_attributes_template(
        mut self,
        json_attributes_template: impl Into<String>,
    ) -> Self {
        self.json_attributes_template = Some(json_attributes_template.into());
        self
    }

    pub fn with_json_attributes_topic(mut self, json_attributes_topic: impl Into<String>) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    pub fn with_max(mut self, max: i32) -> Self {
        self.max = Some(max);
        self
    }

    pub fn with_min(mut self, min: i32) -> Self {
        self.min = Some(min);
        self
    }

    pub fn with_mode(mut self, mode: TextMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_object_id(mut self, object_id: impl Into<String>) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    pub fn with_qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    pub fn with_retain(mut self, retain: bool) -> Self {
        self.retain = Some(retain);
        self
    }

    pub fn with_state_topic(mut self, state_topic: impl Into<String>) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    pub fn with_unique_id(mut self, unique_id: impl Into<String>) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    pub fn with_value_template(mut self, value_template: impl Into<String>) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl<'a> crate::component::ObjectId for Text<'a> {
    fn object_id(&self) -> Option<&str> {
        self.object_id.as_deref()
    }
}

impl<'a> crate::component::Name for Text<'a> {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl<'a> crate::component::ComponentTrait for Text<'a> {
    fn component(&self) -> crate::component::Component {
        crate::component::Component::Text
    }
}

impl<'a> crate::component::NodeId for Text<'a> {
    fn node_id(&self) -> Option<&str> {
        self.device.and_then(|device| device.node_id.as_deref())
    }
}
