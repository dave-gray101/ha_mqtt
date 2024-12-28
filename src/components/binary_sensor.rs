use crate::availability::{Availability, AvailabilityMode};
use crate::device::Device;
use crate::qos::Qos;
use serde::{Deserialize, Serialize};
use serde_inner_serialize::InnerSerializable;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum BinarySensorState {
    On,
    Off,
    Unknown,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, InnerSerializable)]
pub struct BinarySensor<'a> {
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

    pub device_class: Option<BinarySensorClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<BinarySensorEntityCategory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_after: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_update: Option<bool>,

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
    pub off_delay: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    pub state_topic: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl<'a> BinarySensor<'a> {
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

    pub fn with_device_class(mut self, device_class: BinarySensorClass) -> Self {
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

    pub fn with_entity_category(mut self, entity_category: BinarySensorEntityCategory) -> Self {
        self.entity_category = Some(entity_category);
        self
    }

    pub fn with_expire_after(mut self, expire_after: i32) -> Self {
        self.expire_after = Some(expire_after);
        self
    }

    pub fn with_force_update(mut self, force_update: bool) -> Self {
        self.force_update = Some(force_update);
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

    pub fn with_object_id(mut self, object_id: impl Into<String>) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    pub fn with_off_delay(mut self, off_delay: i32) -> Self {
        self.off_delay = Some(off_delay);
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

    pub fn with_payload_off(mut self, payload_off: impl Into<String>) -> Self {
        self.payload_off = Some(payload_off.into());
        self
    }

    pub fn with_payload_on(mut self, payload_on: impl Into<String>) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }

    pub fn with_qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    pub fn with_state_topic(mut self, state_topic: impl Into<String>) -> Self {
        self.state_topic = state_topic.into();
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

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl crate::component::ObjectId for BinarySensor<'_> {
    fn object_id(&self) -> Option<&str> {
        self.object_id.as_deref()
    }
}

impl crate::component::ComponentTrait for BinarySensor<'_> {
    fn component(&self) -> crate::component::Component {
        crate::component::Component::BinarySensor
    }
}

impl crate::component::NodeId for BinarySensor<'_> {
    fn node_id(&self) -> Option<&str> {
        self.device.as_ref().and_then(|d| d.node_id.as_deref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BinarySensorClass {
    #[serde(rename = "None")]
    None,
    Battery,
    BatteryCharging,
    CarbonMonoxide,
    Cold,
    Connectivity,
    Door,
    GarageDoor,
    Gas,
    Heat,
    Light,
    Lock,
    Moisture,
    Motion,
    Occupancy,
    Opening,
    Plug,
    Power,
    Presence,
    Problem,
    Safety,
    Smoke,
    Sound,
    Tamper,
    Update,
    Vibration,
    Window,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum BinarySensorEntityCategory {
    Diagnostic,
}
