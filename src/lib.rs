mod components;
mod query_filters;
mod resources;
mod systems;
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
            (
                systems::ui::setup,
                systems::road::setup,
                systems::car::setup,
            )
                .chain(),
        );
        // Uncomment the next line to enable keyboard input
        //app.add_systems(PreUpdate, keyboard_input::read_input);
        app.add_systems(
            Update,
            (
                systems::ray_cast::update_sprites,
                (systems::car::update_camera_target).in_set(CollisionSystemSet),
                systems::ui::save_handler,
                systems::ui::load_handler,
            ),
        );
        app.add_systems(
            FixedUpdate,
            (
                systems::car::move_cars,
                (systems::car::find_new_camera_target).before(CollisionSystemSet),
                systems::car::despawn_traffic,
                systems::car::spawn_traffic,
                (
                    (systems::car::check_collisions).in_set(CollisionSystemSet),
                    systems::road::move_road,
                )
                    .chain(),
                (systems::ray_cast::cast_rays).in_set(CollisionSystemSet),
                systems::network::update,
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
