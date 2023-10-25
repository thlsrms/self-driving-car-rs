use bevy::log;
use bevy::prelude::*;
use bevy::window::{ExitCondition, WindowResolution};

fn main() {
    let window_size = selfdriving_car::WindowSize(400., 600.);

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(window_size.0, window_size.1),
                        title: "Self-driving test".to_string(),
                        resizable: false,
                        ..default()
                    }),
                    exit_condition: ExitCondition::OnPrimaryClosed,
                    close_when_requested: true,
                })
                .set(log::LogPlugin {
                    filter: "error,wgpu_core=error,wgpu_hal=error".into(),
                    level: log::Level::DEBUG,
                })
                .build(),
        )
        .insert_resource(window_size)
        .add_plugins(selfdriving_car::SelfDrivingCar)
        .run();
}
