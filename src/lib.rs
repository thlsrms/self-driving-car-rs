mod car_systems;
mod components;
mod keyboard_input;
mod network_systems;
mod query_filters;
mod ray_cast_systems;
mod resources;
mod road_systems;
mod ui_systems;
mod utils;
use bevy::prelude::*;
pub use resources::WindowSize;
use resources::{CameraTarget, Config, NetworkConfig};
use std::f32::consts::PI;

const FIXED_DELTA: f32 = 1.0 / 60.0;

pub struct SelfDrivingCar;

impl Plugin for SelfDrivingCar {
    fn build(&self, app: &mut App) {
        let initial_config = Config {
            max_traffic: 18,
            controlllable_cars: 250,
            ..Default::default()
        };
        let network_config = NetworkConfig {
            input_neuron_count: 9,
            input_ray_length: 180.0,
            input_ray_spread: PI * 0.66,
            mutate_factor: 0.5,
            hidden_layers: 1,
            hidden_layers_neuron_count: 6,
            output_neuron_count: 4,
        };

        app.add_systems(
            Startup,
            (ui_systems::setup, road_systems::setup, car_systems::setup).chain(),
        );
        // Uncomment the next line to enable keyboard input
        //app.add_systems(PreUpdate, keyboard_input::read_input);
        app.add_systems(
            Update,
            (
                ray_cast_systems::update_sprites,
                (car_systems::update_camera_target).in_set(CollisionSystemSet),
                ui_systems::save_handler,
                ui_systems::load_handler,
            ),
        );
        app.add_systems(
            FixedUpdate,
            (
                car_systems::move_cars,
                (car_systems::find_new_camera_target).before(CollisionSystemSet),
                car_systems::despawn_traffic,
                car_systems::spawn_traffic,
                (
                    (car_systems::check_collisions).in_set(CollisionSystemSet),
                    road_systems::move_road,
                )
                    .chain(),
                (ray_cast_systems::cast_rays).in_set(CollisionSystemSet),
                network_systems::update,
            )
                .chain(),
        )
        .insert_resource(FixedTime::new_from_secs(FIXED_DELTA))
        .insert_resource(initial_config)
        .insert_resource(network_config)
        .register_type::<components::NetworkLevel>()
        .register_type::<Vec<components::NetworkLevel>>()
        .register_type::<Vec<f32>>()
        .register_type::<Vec<Vec<f32>>>()
        .insert_resource(CameraTarget::default());
    }
}

#[derive(SystemSet, Debug, Hash, Clone, PartialEq, Eq)]
struct CollisionSystemSet;
