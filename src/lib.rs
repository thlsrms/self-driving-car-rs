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

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum AppState {
    #[default]
    Running,
    LoadingNetwork,
}

pub struct SelfDrivingCar;

impl Plugin for SelfDrivingCar {
    fn build(&self, app: &mut App) {
        let initial_config = Config {
            max_traffic: 18,
            controlllable_cars: 250,
            ..Default::default()
        };
        let network_config = NetworkConfig {
            input_neuron_count: 18,
            input_ray_length: 130.0,
            input_ray_spread: PI * 0.9,
            mutate_factor: 0.075,
            hidden_layers: 2,
            hidden_layers_neuron_count: 9,
            output_neuron_count: 4,
        };

        app.insert_resource(FixedTime::new_from_secs(FIXED_DELTA))
            .insert_resource(initial_config)
            .insert_resource(network_config)
            .insert_resource(CameraTarget::default())
            .init_resource::<State<AppState>>();

        app.register_type::<components::NetworkLevel>()
            .register_type::<Vec<components::NetworkLevel>>()
            .register_type::<Vec<f32>>()
            .register_type::<Vec<Vec<f32>>>();

        app.add_event::<LoadNetworkEvent>();

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
            )
                .run_if(state_exists_and_equals(AppState::Running)),
        );
        app.add_systems(
            Update,
            (systems::car::load_network).run_if(state_exists_and_equals(AppState::LoadingNetwork)),
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
                .chain()
                .run_if(state_exists_and_equals(AppState::Running)),
        );
    }
}

#[derive(SystemSet, Debug, Hash, Clone, PartialEq, Eq)]
struct CollisionSystemSet;

#[derive(Event)]
pub struct LoadNetworkEvent;
