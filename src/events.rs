use crate::components;
use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct LoadNetworkEvent(pub Vec<components::NetworkLevel>);
#[derive(Event)]
pub struct ChangeTargetEvent(pub Entity, pub Option<Entity>);
