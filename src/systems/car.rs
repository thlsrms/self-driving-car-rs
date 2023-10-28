use crate::components::{
    CameraFollowMarker, Car, CarCollided, CarsArray, ControllableCarBundle, Controls,
    NeuralNetwork, NewCameraTarget, RayBundle, TrafficCarBundle,
};
use crate::query_filters;
use crate::resources::{CameraTarget, Config, NetworkConfig, RoadProperties, WindowSize};
use crate::utils::lerp;
use bevy::prelude::*;
use bevy::sprite::collide_aabb;
use rand::Rng;
use std::f32::consts::PI;

pub fn setup(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    road: Res<RoadProperties>,
    mut config: ResMut<Config>,
    network_config: Res<NetworkConfig>,
) {
    commands
        .spawn_empty()
        .insert(SpatialBundle::default())
        .insert(CarsArray)
        .with_children(|parent| {
            (0..config.controlllable_cars).for_each(|_| {
                let mut ray_ids: Vec<Entity> = vec![];
                let mut car = parent.spawn(ControllableCarBundle::new(Vec2 {
                    x: road.get_lane_ceter(2),
                    y: -window_size.1 / 4.,
                }));
                car.with_children(|parent| {
                    (0..network_config.input_neuron_count).for_each(|i| {
                        let ray_angle = {
                            let t = if network_config.input_neuron_count == 1 {
                                0.5
                            } else {
                                f32::from(i) / f32::from(network_config.input_neuron_count - 1)
                            };
                            let a = network_config.input_ray_spread / 2.;
                            lerp::<f32, f32>(a, -a, t)
                        };
                        ray_ids.push(
                            parent
                                .spawn(RayBundle::new(network_config.input_ray_length, ray_angle))
                                .remove::<Visibility>()
                                .id(),
                        );
                    });
                });
                let mut network_layers: Vec<u8> = Vec::new();
                network_layers.insert(0, network_config.input_neuron_count);
                (1..usize::from(network_config.hidden_layers)).for_each(|idx| {
                    network_layers.insert(idx, network_config.hidden_layers_neuron_count);
                });
                network_layers.push(network_config.output_neuron_count);

                car.insert(NeuralNetwork::new(
                    &network_layers,
                    ray_ids,
                    network_config.mutate_factor,
                ));
            });

            // Initial traffic - spawn one third of the max traffic
            (0..config.max_traffic / 3).for_each(|i| {
                let random_lane: u8 = rand::thread_rng().gen_range(0..road.lane_count);
                let random_y: f32 =
                    rand::thread_rng().gen_range(0f32..=(f32::from(i) * 100f32)) + 100f32;
                parent.spawn(TrafficCarBundle::new(
                    road.get_lane_ceter(random_lane),
                    random_y,
                ));
                config.current_traffic += 1;
            });
        });

    commands.spawn(Camera2dBundle::default());
}

pub fn move_cars(
    mut car_q: Query<(&mut Car, &mut Transform, Option<&Controls>), Without<CarCollided>>,
    time: Res<FixedTime>,
) {
    car_q.for_each_mut(|(mut car, mut car_xform, car_controls)| {
        let mut rotation_factor = 0.;
        if let Some(control) = car_controls {
            car.speed += car.acceleration * control.acceleration;
        } else {
            car.speed += car.acceleration;
        }

        car.speed = match car.speed {
            speed if speed.abs() < car.friction => 0.,
            speed if speed < 0. => speed + car.friction,
            speed if speed > 0. => speed - car.friction,
            _ => 0.,
        };
        car.speed = car.speed.clamp(-car.max_speed * 0.5, car.max_speed);

        if car.speed != 0. && car.speed.abs() > car.acceleration * 1.5 {
            car.handling =
                f32::to_radians(car.max_handling / (car.speed / 100.)).clamp(-PI * 0.66, PI * 0.66);
            if let Some(control) = car_controls {
                rotation_factor = control.turn_direction;
            }
        }
        car_xform.rotate_z(rotation_factor * car.handling * time.period.as_secs_f32());

        let movement_delta =
            (car_xform.rotation * Vec3::Y) * (car.speed * time.period.as_secs_f32());
        car_xform.translation += movement_delta;
    });
}

pub fn check_collisions(
    mut commands: Commands,
    mut cars_q: Query<(&Transform, &Children, &mut Sprite, Entity), query_filters::ControllableCar>,
    colliders_q: Query<(&Transform, &Sprite), query_filters::Collider>,
    mut camera_target: ResMut<CameraTarget>,
) {
    for (car_xform, car_children, mut car_sprite, car_id) in &mut cars_q {
        for (collider_xform, collider_sprite) in colliders_q.iter() {
            if car_xform.translation.distance(collider_xform.translation)
                >= car_sprite.custom_size.unwrap().y
            {
                continue;
            }
            if let Some(collider_size) = collider_sprite.custom_size {
                if collide_aabb::collide(
                    car_xform.translation,
                    car_sprite.custom_size.unwrap(),
                    collider_xform.translation,
                    collider_size,
                )
                .is_some()
                {
                    let mut car_entity = commands.entity(car_id);
                    car_entity.remove_children(car_children);
                    car_entity.insert(CarCollided);
                    car_entity.remove::<CameraFollowMarker>();
                    for child in car_children {
                        commands.entity(*child).despawn();
                    }
                    car_sprite.color.set_a(50.);
                    car_sprite.color = Color::DARK_GRAY;
                    if Some(car_id) == camera_target.get_curr_target() {
                        camera_target.cleanup();
                        camera_target.remove_target();
                    }
                }
            }
        }
    }
}

pub fn find_new_camera_target(
    mut commands: Commands,
    cars_q: Query<(&Transform, Entity, &Children), query_filters::ControllableCar>,
    mut camera_target: ResMut<CameraTarget>,
) {
    let mut furthermost_y_car: Option<Entity> = camera_target.get_curr_target();
    let mut furthermost_y_value: f32 = match furthermost_y_car {
        Some(car_id) => cars_q.get(car_id).unwrap().0.translation.y,
        None => -1000.0,
    };
    for (car_xform, car_id, _) in cars_q.iter() {
        if furthermost_y_car.is_none() && camera_target.get_old_target().is_none() {
            // There's no target, make the first available as the new target
            commands.entity(car_id).insert(NewCameraTarget);
            camera_target.set_curr_target(car_id);
            // set_curr_target twice to push a value as the old_target
            camera_target.set_curr_target(car_id);
            return;
        }
        if car_xform.translation.y > furthermost_y_value {
            furthermost_y_value = car_xform.translation.y;
            furthermost_y_car = Some(car_id);
        }
    }

    if let Some(curr_target) = camera_target.get_curr_target() {
        if furthermost_y_car.unwrap() != curr_target {
            camera_target.set_curr_target(furthermost_y_car.unwrap());
            commands
                .entity(furthermost_y_car.unwrap())
                .insert(NewCameraTarget);
        }
    }
}

pub fn update_camera_target(
    mut commands: Commands,
    mut camera_target_candidate_q: Query<(&mut Sprite, &Children), query_filters::CameraTransition>,
    mut other_targets_q: Query<(&mut Sprite, &Children, Entity), With<CameraFollowMarker>>,
    mut camera_target: ResMut<CameraTarget>,
) {
    if let Some(new_target) = camera_target.get_curr_target() {
        if let Some(old_target) = camera_target.get_old_target() {
            'disable_old_target: {
                if new_target != old_target {
                    let Ok((mut other_sprite, other_children, other_entity)) =
                        other_targets_q.get_mut(old_target)
                    else {
                        break 'disable_old_target;
                    };
                    commands.entity(other_entity).remove::<CameraFollowMarker>();
                    other_sprite.color = Color::GREEN;
                    other_sprite.color.set_a(0.05);
                    for child in other_children {
                        commands.entity(*child).insert(Visibility::Hidden);
                    }
                }
            }

            let Ok((mut target_sprite, target_children)) =
                camera_target_candidate_q.get_mut(new_target)
            else {
                return;
            };
            commands.entity(new_target).remove::<NewCameraTarget>();
            commands.entity(new_target).insert(CameraFollowMarker);
            target_sprite.color = Color::YELLOW_GREEN;
            target_sprite.color.set_a(1.);
            for child in target_children {
                commands.entity(*child).insert(Visibility::Visible);
            }
            camera_target.cleanup();
        }
    }
}

pub fn despawn_traffic(
    mut commands: Commands,
    camera_q: Query<&Transform, With<Camera2d>>,
    traffic_q: Query<(Entity, &Transform), query_filters::Traffic>,
    window_size: Res<WindowSize>,
    mut options: ResMut<Config>,
) {
    let camera_xform = camera_q.single();
    let y_position_constraints = (
        camera_xform.translation.y - window_size.1,
        camera_xform.translation.y + window_size.1 * 2.,
    );

    for (traffic_car_id, traffic_car_xform) in traffic_q.iter() {
        if !(y_position_constraints.0..=y_position_constraints.1)
            .contains(&traffic_car_xform.translation.y)
        {
            commands.entity(traffic_car_id).despawn();
            options.current_traffic -= 1;
        }
    }
}

pub fn spawn_traffic(
    mut commands: Commands,
    cars_array_q: Query<Entity, With<CarsArray>>,
    camera_q: Query<&Transform, With<Camera2d>>,
    window_size: Res<WindowSize>,
    road: Res<RoadProperties>,
    mut options: ResMut<Config>,
) {
    let camera_xform = camera_q.single();
    let min_y = camera_xform.translation.y + window_size.1;
    let cars_array = cars_array_q.single();

    (0..options.max_traffic - options.current_traffic).for_each(|i| {
        let random_lane: u8 = rand::thread_rng().gen_range(0..road.lane_count);
        let random_y: f32 = rand::thread_rng().gen_range(0f32..=(f32::from(i) * 100f32)) + min_y;
        let new_car = commands
            .spawn(TrafficCarBundle::new(
                road.get_lane_ceter(random_lane),
                random_y,
            ))
            .id();
        commands.entity(cars_array).add_child(new_car);
        options.current_traffic += 1;
    });
}
