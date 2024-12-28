use crate::availability::{Availability, AvailabilityMode};
use crate::device::Device;
use crate::qos::Qos;
use serde::Serialize;
use serde_inner_serialize::InnerSerializable;

#[derive(Debug, Default, Clone, PartialEq, Serialize, InnerSerializable)]
pub struct Button<'a> {
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

    pub device_class: Option<ButtonClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<ButtonEntityCategory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_press: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl<'a> Button<'a> {
    pub fn new(command_topic: String) -> Button<'a> {
        Button {
            command_topic,
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
    pub fn with_command_template(mut self, command_template: impl Into<String>) -> Self {
        self.command_template = Some(command_template.into());
        self
    }
    pub fn with_device(mut self, device: &'a Device) -> Self {
        self.device = Some(device);
        self
    }
    pub fn with_device_class(mut self, device_class: ButtonClass) -> Self {
        self.device_class = Some(device_class);
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
    pub fn with_entity_category(mut self, entity_category: ButtonEntityCategory) -> Self {
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
    pub fn with_payload_available(mut self, payload_available: impl Into<String>) -> Self {
        self.payload_available = Some(payload_available.into());
        self
    }
    pub fn with_payload_not_available(mut self, payload_not_available: impl Into<String>) -> Self {
        self.payload_not_available = Some(payload_not_available.into());
        self
    }
    pub fn with_payload_press(mut self, payload_press: impl Into<String>) -> Self {
        self.payload_press = Some(payload_press.into());
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
    pub fn with_unique_id(mut self, unique_id: impl Into<String>) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}


#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ButtonClass {
    #[serde(rename = "None")]
    None,
    Identify,
    Restart,
    Update,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ButtonEntityCategory {
    Config,
    Diagnostic,
}

impl<'a> crate::component::ObjectId for Button<'a> {
    fn object_id(&self) -> Option<&str> {
        self.object_id.as_deref()
    }
}

impl<'a> crate::component::Name for Button<'a> {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl<'a> crate::component::ComponentTrait for Button<'a> {
    fn component(&self) -> crate::component::Component {
        crate::component::Component::Button
    }
}

impl<'a> crate::component::NodeId for Button<'a> {
    fn node_id(&self) -> Option<&str> {
        self.device.and_then(|device| device.node_id.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_button() {
        let button = Button::new("command_topic".to_string());

        assert_eq!(button.command_topic, "command_topic");
        assert_eq!(button.availability, None);
        assert_eq!(button.availability_mode, None);
        assert_eq!(button.availability_template, None);
        assert_eq!(button.availability_topic, None);
        assert_eq!(button.command_template, None);
        assert_eq!(button.device, None);
        assert_eq!(button.device_class, None);
        assert_eq!(button.enabled_by_default, None);
        assert_eq!(button.encoding, None);
        assert_eq!(button.entity_category, None);
        assert_eq!(button.icon, None);
        assert_eq!(button.json_attributes_template, None);
        assert_eq!(button.json_attributes_topic, None);
        assert_eq!(button.name, None);
        assert_eq!(button.object_id, None);
        assert_eq!(button.payload_available, None);
        assert_eq!(button.payload_not_available, None);
        assert_eq!(button.payload_press, None);
        assert_eq!(button.qos, None);
        assert_eq!(button.retain, None);
        assert_eq!(button.unique_id, None);
    }

    #[test]
    fn test_with_availability() {
        let availability = Availability::new("topic".to_string());
        let button =
            Button::new("command_topic".to_string()).with_availability(availability.clone());

        assert_eq!(button.availability, Some(availability));
    }

    #[test]
    fn test_with_availability_mode() {
        let button = Button::new("command_topic".to_string())
            .with_availability_mode(AvailabilityMode::Latest);

        assert_eq!(button.availability_mode, Some(AvailabilityMode::Latest));
    }

    // Add more tests for other methods...
}
