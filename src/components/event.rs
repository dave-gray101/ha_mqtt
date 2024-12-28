use crate::availability::{Availability, AvailabilityMode};
use crate::device::Device;
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Event<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<AvailabilityMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<&'a Device>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<EventClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EventEntityCategory>,

    pub event_types: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,

    pub state_topic: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum EventClass {
    #[serde(rename = "None")]
    None,
    Button,
    Doorbell,
    Motion,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum EventEntityCategory {
    Diagnostic,
    Config,
}

impl<'a> Event<'a> {
    pub fn new(state_topic: impl Into<String>) -> Self {
        Self {
            state_topic: state_topic.into(),
            ..Default::default()
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
    pub fn with_device(mut self, device: &'a Device) -> Self {
        self.device = Some(device);
        self
    }
    pub fn with_device_class(mut self, device_class: EventClass) -> Self {
        self.device_class = Some(device_class);
        self
    }
    pub fn with_entity_category(mut self, entity_category: EventEntityCategory) -> Self {
        self.entity_category = Some(entity_category);
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
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn with_object_id(mut self, object_id: impl Into<String>) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn with_payload_available(mut self, payload: impl Into<String>) -> Self {
        self.payload_available = Some(payload.into());
        self
    }
    pub fn with_payload_not_available(mut self, payload: impl Into<String>) -> Self {
        self.payload_not_available = Some(payload.into());
        self
    }
    pub fn with_unique_id(mut self, unique_id: impl Into<String>) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
    pub fn with_value_template(mut self, template: impl Into<String>) -> Self {
        self.value_template = Some(template.into());
        self
    }
    pub fn with_event_types(mut self, event_types: Vec<String>) -> Self {
        self.event_types = event_types;
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
    pub fn with_state_topic(mut self, state_topic: impl Into<String>) -> Self {
        self.state_topic = state_topic.into();
        self
    }
}

impl<'a> crate::component::ObjectId for Event<'a> {
    fn object_id(&self) -> Option<&str> {
        self.object_id.as_deref()
    }
}

impl<'a> crate::component::Name for Event<'a> {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl<'a> crate::component::ComponentTrait for Event<'a> {
    fn component(&self) -> crate::component::Component {
        crate::component::Component::Event
    }
}

impl<'a> crate::component::NodeId for Event<'a> {
    fn node_id(&self) -> Option<&str> {
        self.device.and_then(|device| device.node_id.as_deref())
    }
}
