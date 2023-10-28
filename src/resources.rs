use bevy::prelude::{Entity, Resource};

#[derive(Resource, Default)]
pub struct Config {
    pub max_traffic: u8,
    pub current_traffic: u8,
    pub controlllable_cars: u16,
}

#[derive(Resource, Default)]
pub struct NetworkConfig {
    pub hidden_layers: u8,
    pub hidden_layers_neuron_count: u8,
    pub input_neuron_count: u8,
    pub input_ray_length: f32,
    pub input_ray_spread: f32,
    #[allow(unused)]
    pub mutate_factor: f32,
    pub output_neuron_count: u8,
}

#[derive(Resource)]
pub struct WindowSize(pub f32, pub f32);

#[derive(Resource, Clone, Copy)]
pub struct RoadProperties {
    pub lane_count: u8,
    pub width: f32,
}

impl RoadProperties {
    pub fn get_lane_ceter(self, lane_idx: u8) -> f32 {
        let lane_width = self.width / f32::from(self.lane_count);
        //(lane_width * lane_idx as f32 - self.width) + lane_width / 2.
        (lane_width * f32::from(lane_idx)) - self.width / 2. + lane_width / 2.
    }
}

/// Stores the entity information necessary for the camera transition
#[derive(Resource, Default, Debug)]
pub struct CameraTarget(Option<Entity>);

impl CameraTarget {
    pub fn set_target(&mut self, new_target: Entity) {
        self.0 = Some(new_target);
    }

    pub fn get_target(&self) -> Option<Entity> {
        self.0
    }

    pub fn remove_target(&mut self) {
        self.0 = None;
    }
}
