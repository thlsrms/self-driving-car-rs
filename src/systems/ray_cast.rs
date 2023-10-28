use crate::components::{Ray, StaticCollider};
use crate::query_filters;
use bevy::prelude::{Changed, Children, Color, Entity, Query, Sprite, Transform, Vec2, Visibility};

pub fn cast_rays(
    cars_q: Query<(&Transform, &Children), query_filters::ControllableCar>,
    mut rays_q: Query<(&mut Ray, Entity)>,
    mut colliders_q: Query<(&Transform, &Sprite, Entity, &mut StaticCollider)>,
) {
    for (car_xform, car_children) in cars_q.iter() {
        for &child in car_children {
            let Ok((mut ray, ray_id)) = rays_q.get_mut(child) else {
                continue;
            };

            for (collider_xform, collider_sprite, collider_id, mut static_collider) in
                &mut colliders_q
            {
                // No reason to check if the collider is too far from the ray
                if car_xform.translation.distance(collider_xform.translation) >= ray.length * 1.5 {
                    continue;
                }
                if let Some(collider_size) = collider_sprite.custom_size {
                    let collider_index = ray.collisions.iter().position(|(e, _)| *e == collider_id);

                    match ray.get_intersecting_point(
                        car_xform,
                        &collider_xform.translation,
                        collider_size,
                    ) {
                        // If let guard could be useful in here
                        // they are still experimental: rust-lang/rust/issues/51114
                        Some(intersection) if matches!(collider_index, Some(_collider_index)) => {
                            let collider_index = collider_index.unwrap();
                            // Update existing colliding entity with new collided position
                            if intersection.1 != ray.collisions[collider_index].1 {
                                ray.collisions.remove(collider_index);
                                ray.collisions.push((collider_id, intersection.1));
                            }
                        }
                        Some(intersection) => {
                            static_collider.colliding_with.push(ray_id);
                            ray.collisions.push((collider_id, intersection.1));
                        }
                        None => {
                            if let Some(collider_index) = collider_index {
                                ray.collisions.remove(collider_index);
                                let ray_index = static_collider
                                    .colliding_with
                                    .iter()
                                    .position(|e| *e == ray_id)
                                    .unwrap();
                                static_collider.colliding_with.remove(ray_index);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn update_sprites(mut rays_q: Query<(&mut Sprite, &Ray, &Visibility), Changed<Ray>>) {
    for (mut ray_sprite, ray, visibility) in &mut rays_q {
        if ray.collisions.is_empty() {
            ray_sprite.color = Color::BLUE;
            ray_sprite.custom_size = Some(Vec2 {
                x: 2.0,
                y: ray.length,
            });
            continue;
        }

        if visibility != Visibility::Visible {
            continue;
        }

        let min_dist = ray
            .collisions
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        ray_sprite.custom_size = Some(Vec2 {
            x: 2.0,
            y: ray.length * min_dist.1,
        });
        ray_sprite.color = Color::RED;
    }
}
