use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct LoadNetworkEvent;
#[derive(Event)]
pub struct ChangeTargetEvent(pub Entity, pub Option<Entity>);
