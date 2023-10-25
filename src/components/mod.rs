mod car;
mod network;
mod ray;
use bevy::prelude::{Component, Entity};

pub(super) use car::{Car, ControllableCarBundle, TrafficCar, TrafficCarBundle};
pub(super) use network::{NetworkLevel, NeuralNetwork};
pub(super) use ray::{Ray, RayBundle};

#[derive(Component, Default)]
pub(super) struct StaticCollider {
    pub colliding_with: Vec<Entity>,
}
#[derive(Component, Default, Debug)]
pub(super) struct Controls {
    pub acceleration: f32,
    pub turn_direction: f32,
}
#[derive(Component)]
pub(super) struct CarsArray;
#[derive(Component)]
pub(super) struct CameraFollowMarker;
#[derive(Component)]
pub(super) struct NewCameraTarget;
#[derive(Component)]
pub(super) struct CarCollided;

/* Road components    */
#[derive(Component)]
pub(super) struct Road;
/// Marker struct to filter and move the road markings
#[derive(Component)]
pub(super) struct RoadLine;
/// Marker struct for the road background
#[derive(Component)]
pub(super) struct Pavement;

/* UI components      */
#[derive(Component)]
pub(super) struct SaveButton;
#[derive(Component)]
pub(super) struct LoadButton;
