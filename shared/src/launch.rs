use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LaunchParams {
    pub distance: f32,
    pub max_speed: f32,
}

impl LaunchParams {

}
