use serde::Serialize;

use serde_inner_serialize::InnerSerializable;

#[derive(Debug, Default, Clone, PartialEq, Serialize, InnerSerializable)]
pub struct Sensor<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<crate::availability::Availability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<crate::availability::AvailabilityMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<&'a crate::device::Device>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<SensorClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<SensorEntityCategory>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reset_value_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub suggested_display_precision: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<crate::qos::Qos>,

    pub state_topic: String,

    pub unit_of_measurement: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl<'a> Sensor<'a> {
    pub fn new(state_topic: impl Into<String>) -> Self {
        Self {
            state_topic: state_topic.into(),
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
    pub fn with_device(mut self, device: &'a crate::device::Device) -> Self {
        self.device = Some(device);
        self
    }
    pub fn with_device_class(mut self, device_class: SensorClass) -> Self {
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
    pub fn with_entity_category(mut self, entity_category: SensorEntityCategory) -> Self {
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
    pub fn with_json_attributes(mut self, json_attributes_topic: impl Into<String>) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }
    pub fn with_last_reset_value_template(
        mut self,
        last_reset_value_template: impl Into<String>,
    ) -> Self {
        self.last_reset_value_template = Some(last_reset_value_template.into());
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
    pub fn with_suggested_display_precision(mut self, suggested_display_precision: i32) -> Self {
        self.suggested_display_precision = Some(suggested_display_precision);
        self
    }
    pub fn with_qos(mut self, qos: crate::qos::Qos) -> Self {
        self.qos = Some(qos);
        self
    }
    pub fn with_unit_of_measurement(mut self, unit_of_measurement: impl Into<String>) -> Self {
        self.unit_of_measurement = Some(unit_of_measurement.into());
        self
    }
    pub fn with_value_template(mut self, value_template: impl Into<String>) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl<'a> crate::component::ObjectId for Sensor<'a> {
    fn object_id(&self) -> Option<&str> {
        self.object_id.as_deref()
    }
}

impl<'a> crate::component::Name for Sensor<'a> {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl<'a> crate::component::ComponentTrait for Sensor<'a> {
    fn component(&self) -> crate::component::Component {
        crate::component::Component::Sensor
    }
}

impl<'a> crate::component::NodeId for Sensor<'a> {
    fn node_id(&self) -> Option<&str> {
        self.device.and_then(|device| device.node_id.as_deref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum SensorEntityCategory {
    Diagnostic,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum SensorClass {
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
    Enum,
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
