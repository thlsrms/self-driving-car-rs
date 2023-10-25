use super::{Controls, StaticCollider};
use bevy::prelude::{
    default, Bundle, Color, Component, Sprite, SpriteBundle, Transform, Vec2, Vec3,
};
use rand::Rng;

#[derive(Component)]
pub struct Car {
    pub acceleration: f32,
    pub friction: f32,
    pub handling: f32,
    pub max_handling: f32,
    pub max_speed: f32,
    pub speed: f32,
}

impl Car {
    pub fn new(max_speed: f32) -> Self {
        Car {
            acceleration: 3.,
            friction: 1.,
            handling: 0.0,
            max_handling: 35.,
            max_speed,
            speed: 0.0,
        }
    }
}

#[derive(Bundle)]
pub struct ControllableCarBundle {
    car: Car,
    controls: Controls,
    sprite: SpriteBundle,
}

impl ControllableCarBundle {
    pub fn new(car_max_speed: f32, position: Vec2) -> Self {
        Self {
            car: Car::new(car_max_speed),
            controls: Controls::default(),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba_u8(55, 150, 55, 125),
                    custom_size: Some(Vec2 { x: 30.0, y: 50.0 }),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: position.x,
                        y: position.y,
                        z: 0.0,
                    },
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct TrafficCar;

#[derive(Bundle)]
pub struct TrafficCarBundle {
    car: Car,
    sprite: SpriteBundle,
    collider: StaticCollider,
    traffic_car: TrafficCar,
}

impl TrafficCarBundle {
    pub fn new(position_x: f32, position_y: f32) -> Self {
        let random_speed: f32 = rand::thread_rng().gen_range(60f32..=120f32);
        Self {
            car: Car::new(random_speed),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::BEIGE,
                    custom_size: Some(Vec2 { x: 30.0, y: 50.0 }),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: position_x,
                        y: position_y,
                        z: 0.0,
                    },
                    ..default()
                },
                ..default()
            },
            collider: StaticCollider::default(),
            traffic_car: TrafficCar,
        }
    }
}
