 # Home Assistant MQTT Discovery type definitions  
 Useful for creating Home Assistant MQTT Discovery messages.  
 Not all types are implemented yet.  

 All types implement the `Serialize` trait from the `serde` crate to allow creating the required JSON message for Home Assistant MQTT Discovery.
 Custom trait of `Discoverable` and `State` allow the creatiion of the config and state topics respectively.
 See: https://www.home-assistant.io/integrations/mqtt/ for more information on Home Assistant MQTT settings.  

 