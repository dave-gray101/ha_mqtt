use serde::{Deserialize, Serialize};

#[allow(clippy::declare_interior_mutable_const)]
const PREFIX: std::cell::OnceCell<&str> = std::cell::OnceCell::new();

pub trait ObjectId {
    fn object_id(&self) -> &str;
}

pub trait ComponentTrait {
    fn component(&self) -> crate::component::Component;

    fn prefix(&self) -> &'static str;
}

impl dyn ComponentTrait {
    fn prefix(&self) -> &'static str {
        #[allow(clippy::borrow_interior_mutable_const)]
        PREFIX.get_or_init(|| option_env!("HA_MQTT_PREFIX").unwrap_or("homeassistant"))
    }
}

pub trait NodeId {
    fn node_id(&self) -> Option<&str>;
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Component {
    AlarmControlPanel,
    BinarySensor,
    Button,
    Camera,
    Cover,
    DeviceTracker,
    Event,
    Fan,
    Humidifier,
    Image,
    Climate,
    LawnMover,
    Light,
    Lock,
    Notify,
    Number,
    Scene,
    Select,
    Sensor,
    Siren,
    Switch,
    Text,
    Update,
    Vacuum,
    Valve,
    WaterHeater,
}

impl std::fmt::Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = to_snake_case(&format!("{:?}", self));
        write!(f, "{}", str)
    }
}
fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_serialize_component() {
        let component = Component::AlarmControlPanel;
        let serialized = serde_json::to_string(&component).unwrap();
        assert_eq!(serialized, "\"alarm_control_panel\"");
    }

    #[test]
    fn should_write_to_string() {
        let component = Component::AlarmControlPanel;
        let string = component.to_string();
        assert_eq!(string, "alarm_control_panel");
    }
}
