use crate::components::Controls;
use bevy::prelude::*;

#[allow(unused)]
pub(super) fn read_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut controls_q: Query<&mut Controls>,
) {
    for mut controls in controls_q.iter_mut() {
        controls.acceleration = 0.;
        controls.turn_direction = 0.;
        keyboard_input
            .pressed(KeyCode::Up)
            .then(|| controls.acceleration += 1.);
        keyboard_input
            .pressed(KeyCode::Down)
            .then(|| controls.acceleration -= 1.);
        keyboard_input
            .pressed(KeyCode::Left)
            .then(|| controls.turn_direction += 1.);
        keyboard_input
            .pressed(KeyCode::Right)
            .then(|| controls.turn_direction -= 1.);
    }
}
