use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TelemetryData {
    pub field_name: String,
    pub field_value: f32,
    pub value_upper: f32,
    pub value_lower: f32,
}

impl TelemetryData {
    pub fn new(field_name: String, field_value: f32, value_upper: f32, value_lower: f32) -> Self {
        Self {
            field_name,
            field_value,
            value_upper,
            value_lower,
        }
    }
}
