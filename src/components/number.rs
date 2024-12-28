use crate::availability::{Availability, AvailabilityMode};
use crate::device::Device;
use crate::qos::Qos;
use serde::Serialize;
use serde_inner_serialize::InnerSerializable;

#[derive(Debug, Default, Clone, PartialEq, Serialize, InnerSerializable)]
pub struct Number<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<AvailabilityMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    pub command_topic: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<&'a Device>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<NumberClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<NumberEntityCategory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<NumberMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_reset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    pub unit_of_measurement: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl<'a> Number<'a> {
    pub fn new(command_topic: String, unit_of_measurement: String) -> Number<'a> {
        Number {
            command_topic,
            unit_of_measurement,
            ..Default::default()
        }
    }

    pub fn with_availability(mut self, availability: Availability) -> Self {
        self.availability = Some(availability);
        self
    }

    pub fn with_availability_topic(mut self, availability_topic: impl Into<String>) -> Self {
        self.availability_topic = Some(availability_topic.into());
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

    pub fn with_command_template(mut self, command_template: impl Into<String>) -> Self {
        self.command_template = Some(command_template.into());
        self
    }

    pub fn with_device(mut self, device: &'a Device) -> Self {
        self.device = Some(device);
        self
    }

    pub fn with_device_class(mut self, device_class: NumberClass) -> Self {
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

    pub fn with_entity_category(mut self, entity_category: NumberEntityCategory) -> Self {
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

    pub fn with_json_attributes(mut self, json_attributes_topic: impl Into<String>) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    pub fn with_min(mut self, min: i32) -> Self {
        self.min = Some(min);
        self
    }

    pub fn with_max(mut self, max: i32) -> Self {
        self.max = Some(max);
        self
    }

    pub fn with_mode(mut self, mode: NumberMode) -> Self {
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

    pub fn with_optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    pub fn with_payload_reset(mut self, payload_reset: impl Into<String>) -> Self {
        self.payload_reset = Some(payload_reset.into());
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

    pub fn with_step(mut self, step: i32) -> Self {
        self.step = Some(step);
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

#[derive(Debug, Serialize, PartialEq, Clone, Copy)]
#[serde(untagged, rename_all = "snake_case")]
pub enum NumberClass {
    #[serde(rename = "None")]
    None,
    ApparentPower,
    Aql,
    AtmosphericPressure,
    Battery,
    CarbonDioxide,
    CarbonMonoxide,
    Current,
    DataRate,
    DataSize,
    Date,
    Distance,
    Duration,
    Energy,
    EnergyStorage,
    Frequency,
    Gas,
    Humidity,
    Illuminance,
    Irradiance,
    Moisture,
    Monetary,
    NitrogenDioxide,
    NitrogenMonoxide,
    NitrousOxide,
    Ozone,
    Ph,
    Pm1,
    Pm10,
    Pm25,
    Power,
    PowerFactor,
    Precipitation,
    PrecipitationIntensity,
    Pressure,
    ReactivePower,
    SignalStrength,
    SoundPressure,
    Speed,
    SulfurDioxide,
    Temperature,
    Timestamp,
    VolatileOrganicCompound,
    VolativeOrganicCompoundParts,
    Voltage,
    Volume,
    VolumeFlowRate,
    Water,
    Weight,
    WindSpeed,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum NumberEntityCategory {
    Diagnostic,
    Config,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum NumberMode {
    Auto,
    Box,
    Slider,
}

impl<'a> crate::component::ObjectId for Number<'a> {
    fn object_id(&self) -> Option<&str> {
        self.object_id.as_deref()
    }
}

impl<'a> crate::component::ComponentTrait for Number<'a> {
    fn component(&self) -> crate::component::Component {
        crate::component::Component::Number
    }
}

impl<'a> crate::component::NodeId for Number<'a> {
    fn node_id(&self) -> Option<&str> {
        self.device.and_then(|device| device.node_id.as_deref())
    }
}
