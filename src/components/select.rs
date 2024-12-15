use serde::Serialize;
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Select<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<crate::availability::Availability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<crate::availability::AvailabilityMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    pub command_topic: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<&'a crate::device::Device>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

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
    pub optimistic: Option<bool>,

    pub options: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<crate::qos::Qos>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,

}

impl<'a> Select<'a> {
    pub fn new(command_topic: impl Into<String>, options: Vec<String>) -> Self {
        Self {
            command_topic: command_topic.into(),
            options: options.into(),
            ..Default::default()
        }
    }
    pub fn with_unique_id(mut self, unique_id: impl Into<String>) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
    pub fn with_availability(mut self, availability: crate::availability::Availability) -> Self {
        self.availability = Some(availability);
        self
    }
    pub fn with_availability_mode(
        mut self,
        availability_mode: crate::availability::AvailabilityMode,
    ) -> Self {
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
    pub fn with_command_topic(mut self, command_topic: impl Into<String>) -> Self {
        self.command_topic = command_topic.into();
        self
    }
    pub fn with_device(mut self, device: &'a crate::device::Device) -> Self {
        self.device = Some(device);
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
    pub fn with_entity_category(mut self, entity_category: impl Into<String>) -> Self {
        self.entity_category = Some(entity_category.into());
        self
    }
    pub fn with_entity_picture(mut self, entity_picture: impl Into<String>) -> Self {
        self.entity_picture = Some(entity_picture.into());
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
    pub fn with_json_attributes(mut self, json_attributes_topic: impl Into<String>) -> Self {
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
    pub fn with_optimistic(mut self, optimistic: impl Into<bool>) -> Self {
        self.optimistic = Some(optimistic.into());
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
    pub fn with_qos(mut self, qos: crate::qos::Qos) -> Self {
        self.qos = Some(qos);
        self
    }
    pub fn with_retain(mut self, retain: impl Into<bool>) -> Self {
        self.retain = Some(retain.into());
        self
    }
    pub fn with_state_topic(mut self, state_topic: impl Into<String>) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }
    pub fn with_value_template(mut self, value_template: impl Into<String>) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl<'a> crate::discoverable::ObjectId for Select<'a> {
    fn object_id(&self) -> &str {
        self.object_id.as_ref().unwrap()
    }
}

impl<'a> crate::discoverable::Component for Select<'a> {
    fn component(&self) -> crate::component::Component {
        crate::component::Component::Select
    }
}

impl<'a> crate::discoverable::NodeId for Select<'a> {
    fn node_id(&self) -> Option<&str> {
        self.device.and_then(|device| device.node_id.as_deref())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_select() {
        let select = Select::new("command_topic".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(select.command_topic, "command_topic");
        assert_eq!(select.options[1], "b");

        assert_eq!(select.availability, None);
        assert_eq!(select.availability_mode, None);
        assert_eq!(select.availability_template, None);
        assert_eq!(select.availability_topic, None);
        assert_eq!(select.command_template, None);
        assert_eq!(select.device, None);
        assert_eq!(select.enabled_by_default, None);
        assert_eq!(select.encoding, None);
        assert_eq!(select.entity_category, None);
        assert_eq!(select.entity_picture, None);
        assert_eq!(select.icon, None);
        assert_eq!(select.json_attributes_template, None);
        assert_eq!(select.json_attributes_topic, None);
        assert_eq!(select.name, None);
        assert_eq!(select.object_id, None);
        assert_eq!(select.payload_available, None);
        assert_eq!(select.payload_not_available, None);
        assert_eq!(select.qos, None);
        assert_eq!(select.retain, None);
        assert_eq!(select.unique_id, None);
    }
}