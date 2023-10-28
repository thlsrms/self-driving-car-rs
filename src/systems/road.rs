use crate::components::*;
use crate::query_filters;
use crate::resources::{CameraTarget, RoadProperties, WindowSize};
use bevy::prelude::*;

pub fn setup(world: &mut World) {
    let window_size = world.remove_resource::<WindowSize>().unwrap();
    let road = RoadProperties {
        lane_count: 6,
        width: window_size.0, // / 2.,
    };

    world.insert_resource(road);

    world
        .spawn_empty()
        .insert(SpatialBundle::default())
        .insert(Road)
        .with_children(|parent| {
            // background
            parent.spawn((
                Pavement,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb_u8(80, 80, 80),
                        custom_size: Some(Vec2 {
                            x: road.width,
                            y: window_size.1 * 2.,
                        }),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: 0., //-road.width / 2.,
                            y: 0.,
                            z: -10.,
                        },
                        ..default()
                    },
                    ..default()
                },
            ));

            // lanes
            let dash_size = 40.;
            let dash_y_count = (window_size.1 as u16 * 2) / dash_size as u16;
            let lane_width = road.width / (f32::from(road.lane_count));

            (0..=road.lane_count).for_each(|i| {
                for j in 0..dash_y_count {
                    let y = if i == 0 || i == road.lane_count || j % 2 == 0 {
                        ((f32::from(j) * dash_size) - window_size.1) + dash_size / 2.
                    } else {
                        continue;
                    };

                    let mut road_line = parent.spawn((
                        RoadLine,
                        SpriteBundle {
                            sprite: Sprite {
                                color: if i == 0 || i == road.lane_count {
                                    Color::BLACK
                                } else {
                                    Color::rgb_u8(185, 185, 185)
                                },
                                custom_size: Some(Vec2 {
                                    x: 4.,
                                    y: dash_size,
                                }),
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3 {
                                    //x: -road.width / (road.lane_count as f32) * i as f32,
                                    x: lane_width * f32::from(i) - road.width / 2.,
                                    y,
                                    z: -9.,
                                },
                                ..default()
                            },
                            ..default()
                        },
                    ));

                    if i == 0 || i == road.lane_count {
                        // road margins take a static collider
                        road_line.insert(StaticCollider::default());
                    }
                }
            });
        });

    world.insert_resource(window_size);
}

pub fn move_road(
    mut dashes_q: Query<&mut Transform, With<RoadLine>>,
    mut camera_q: Query<&mut Transform, (With<Camera2d>, Without<RoadLine>)>,
    mut pavement_q: Query<&mut Transform, query_filters::Pavement>,
    car_q: Query<Option<&Transform>, query_filters::CameraTarget>,
    window_size: Res<WindowSize>,
    camera_target: Res<CameraTarget>,
) {
    let mut pavement_xform = pavement_q.single_mut();
    let mut camera_xform = camera_q.single_mut();
    if car_q.is_empty() || camera_target.get_curr_target().is_none() {
        return;
    }
    // We can unwrap() here since we would return if camera_target was None
    let car_xform = match car_q.get(camera_target.get_curr_target().unwrap()) {
        Ok(Some(car_xform)) => car_xform,
        Ok(None) => return,
        Err(_) => return,
    };
    camera_xform.translation.y = car_xform.translation.y + window_size.1 / 4.;
    pavement_xform.translation.y = camera_xform.translation.y;

    let y_position_constraints = (
        camera_xform.translation.y - window_size.1,
        camera_xform.translation.y + window_size.1,
    );

    for mut dash_xform in &mut dashes_q {
        if (y_position_constraints.0..=y_position_constraints.1).contains(&dash_xform.translation.y)
        {
            continue;
        }

        if dash_xform.translation.y < y_position_constraints.0 {
            dash_xform.translation.y += window_size.1 * 2.;
        } else if dash_xform.translation.y > y_position_constraints.1 {
            dash_xform.translation.y += -window_size.1 * 2.;
        }
    }
}
