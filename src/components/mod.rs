mod car;
mod network;
mod ray;
use bevy::prelude::{Component, Entity};

pub use car::{Car, ControllableCarBundle, TrafficCar, TrafficCarBundle};
pub use network::{NetworkLevel, NeuralNetwork};
pub use ray::{Ray, RayBundle};

#[derive(Component, Default)]
pub struct StaticCollider {
    pub colliding_with: Vec<Entity>,
}
#[derive(Component, Default, Debug)]
pub struct Controls {
    pub acceleration: f32,
    pub turn_direction: f32,
}
#[derive(Component)]
pub struct CarsArray;
#[derive(Component)]
pub struct CameraFollowMarker;
#[derive(Component)]
pub struct NewCameraTarget;
#[derive(Component)]
pub struct CarCollided;

/* Road components    */
#[derive(Component)]
pub struct Road;
/// Marker struct to filter and move the road markings
#[derive(Component)]
pub struct RoadLine;
/// Marker struct for the road background
#[derive(Component)]
pub struct Pavement;

/* UI components      */
#[derive(Component)]
pub struct SaveButton;
#[derive(Component)]
pub struct LoadButton;
