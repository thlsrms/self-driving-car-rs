use crate::components::{Ray, *};
use bevy::prelude::*;

/// For each network, use the children rays offset values as initial input for the controls
pub fn update(
    mut commands: Commands,
    mut controls_q: Query<(&mut Controls, &mut NeuralNetwork, Entity), Without<CarCollided>>,
    rays_q: Query<&Ray>,
) {
    'control_loop: for (mut controls, mut brain, entity) in &mut controls_q {
        let mut input_offsets: Vec<f32> = vec![];
        for (idx, ray) in brain.input_rays.iter().enumerate() {
            if let Ok(r) = rays_q.get(*ray) {
                let offset = if r.collisions.is_empty() {
                    -1.
                } else {
                    1. - r
                        .collisions
                        .iter()
                        .min_by(|a, b| a.1.total_cmp(&b.1))
                        .unwrap()
                        .1
                };
                input_offsets.insert(idx, offset);
            } else {
                commands.entity(entity).despawn();
                continue 'control_loop;
            };
        }
        let outputs = brain.feed_forward(&input_offsets);

        controls.acceleration = 0.;
        controls.turn_direction = 0.;
        if outputs.len() == 4 {
            controls.acceleration += outputs[0];
            controls.acceleration -= outputs[3];
            controls.turn_direction += outputs[1];
            controls.turn_direction -= outputs[2];
        }
    }
}
