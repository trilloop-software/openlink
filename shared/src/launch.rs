use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LaunchParams {
    pub distance: Option<f32>,
    pub max_speed: Option<f32>,
}

impl LaunchParams {

}
