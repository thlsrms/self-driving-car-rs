use crate::components;
use bevy::prelude::*;

pub(super) type ControllableCar = (
    With<components::Car>,
    With<components::Controls>,
    Without<components::CarCollided>,
);
pub(super) type CameraTransition = (
    With<components::NewCameraTarget>,
    Without<components::CameraFollowMarker>,
);
pub(super) type Collider = (
    With<components::StaticCollider>,
    Without<components::Controls>,
);
pub(super) type Traffic = (
    With<components::Car>,
    With<components::TrafficCar>,
    Without<components::Controls>,
);
pub(super) type Pavement = (
    With<components::Pavement>,
    Without<Camera2d>,
    Without<components::RoadLine>,
);
pub(super) type CameraTarget = (
    With<components::CameraFollowMarker>,
    Without<Camera2d>,
    Without<components::RoadLine>,
    Without<components::Pavement>,
);
pub(super) type SaveButton = (
    Changed<Interaction>,
    With<Button>,
    With<components::SaveButton>,
);
pub(super) type LoadButton = (
    Changed<Interaction>,
    With<Button>,
    With<components::LoadButton>,
);
