use crate::components::*;
use crate::query_filters;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::de::DeserializeSeed;
use bevy::reflect::serde::{TypedReflectDeserializer, TypedReflectSerializer};
use bevy::reflect::TypeRegistration;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                margin: UiRect {
                    left: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(new_button(140.0))
                .insert(LoadButton)
                .with_children(|parent| {
                    parent.spawn(new_text("Load Recent", &font));
                });
            parent
                .spawn(new_button(140.0))
                .insert(SaveButton)
                .with_children(|parent| {
                    parent.spawn(new_text("Save Current", &font));
                });
        });
}

pub fn save_handler(
    mut interaction_q: Query<(&Interaction, &mut BorderColor), query_filters::SaveButton>,
    brain_q: Query<Option<&NeuralNetwork>, (With<CameraFollowMarker>, Without<CarCollided>)>,
    type_registry: Res<AppTypeRegistry>,
) {
    // FIXME: Need a better filter for the brain query, it should never match Multiple Entities
    if brain_q.is_empty() {
        return;
    }
    let Some(brain) = brain_q.single() else {
        return;
    };

    for (interaction, mut border_color) in &mut interaction_q {
        match interaction {
            Interaction::Pressed => {
                border_color.0 = Color::YELLOW_GREEN;
                // Serialize the "brain"
                // FIXME: Aparently only one level is being serialized
                let type_registry = type_registry.read();
                let brain_serialized = ron::ser::to_string_pretty(
                    &TypedReflectSerializer::new(&brain.levels, &type_registry),
                    ron::ser::PrettyConfig::default(),
                )
                .unwrap();
                std::fs::write("brain", brain_serialized).unwrap();
                // TODO: Add date and timestamp to the file name
                // save into a file inside the path assets/brain/TIMESTAMP
                // and maybe rename the last file to old_TIMESTAMP
            }
            Interaction::Hovered => {
                border_color.0 = Color::CYAN;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn load_handler(
    mut interaction_q: Query<(&Interaction, &mut BorderColor), query_filters::LoadButton>,
    type_registry: Res<AppTypeRegistry>,
) {
    for (interaction, mut border_color) in &mut interaction_q {
        match interaction {
            Interaction::Pressed => {
                border_color.0 = Color::YELLOW_GREEN;
                let Some(brain) = (match std::fs::read_to_string("brain") {
                    Ok(brain_serialized) => {
                        let type_registry = type_registry.read();
                        let registration = TypeRegistration::of::<Vec<NetworkLevel>>();
                        let reflect_deserializer =
                            TypedReflectDeserializer::new(&registration, &type_registry);
                        let mut ron_deserializer =
                            ron::de::Deserializer::from_str(&brain_serialized).unwrap();
                        let brain_reflection =
                            match reflect_deserializer.deserialize(&mut ron_deserializer) {
                                Ok(t) => t,
                                Err(e) => {
                                    dbg!("called `Result::unwrap()` on an `Err` value {}", &e);
                                    Box::new(())
                                }
                            };
                        <Vec<NetworkLevel> as FromReflect>::from_reflect(&*brain_reflection)
                    }
                    Err(e) => {
                        dbg!("Error loading brain: {}", e);
                        None
                    }
                }) else {
                    dbg!("Failed to reconstruct brain from reflection");
                    return;
                };
                dbg!(brain);
                // TODO: "Reset" the app, update the NetworkConfig with the loaded brain
                // remove all entities inside the CarsArray and respawn the cars with the
                // loaded brain to the neural network
            }
            Interaction::Hovered => {
                border_color.0 = Color::CYAN;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn new_button(width: f32) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(width),
            height: Val::Px(30.0),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: BackgroundColor(Color::ANTIQUE_WHITE),
        ..default()
    }
}

fn new_text(text: &str, font: &Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: font.clone(),
            font_size: 22.0,
            color: Color::BLACK,
        },
    )
}
