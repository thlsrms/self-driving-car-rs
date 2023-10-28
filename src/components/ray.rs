use bevy::prelude::{
    Bundle, Color, Component, Entity, Quat, Sprite, SpriteBundle, Transform, Vec2, Vec3,
};
use bevy::sprite::Anchor;

#[derive(Component)]
pub struct Ray {
    pub length: f32,
    angle: f32,
    /// Colliding entity and point of the collision in the ray
    pub collisions: Vec<(Entity, f32)>,
}

impl Ray {
    pub fn get_intersecting_point(
        &self,
        car_xform: &Transform,
        target_pos: &Vec3,
        target_size: Vec2,
    ) -> Option<(Vec2, f32)> {
        let rs = car_xform.translation;
        let re = car_xform.translation
            + ((Quat::from_rotation_z(self.angle) * car_xform.rotation) * Vec3::Y) * self.length;
        let t_min = target_pos.truncate() - target_size / 2.0;
        let t_max = target_pos.truncate() + target_size / 2.0;
        let target_segments: [(Vec2, Vec2); 4] = [
            (
                t_min,
                Vec2 {
                    x: t_min.x,
                    y: t_max.y,
                },
            ),
            (
                Vec2 {
                    x: t_min.x,
                    y: t_max.y,
                },
                t_max,
            ),
            (
                t_max,
                Vec2 {
                    x: t_max.x,
                    y: t_min.y,
                },
            ),
            (
                Vec2 {
                    x: t_max.x,
                    y: t_min.y,
                },
                t_min,
            ),
        ];

        let mut colliding_points: Vec<(Vec2, f32)> = vec![];

        for target_segment in target_segments {
            let ts = target_segment.0;
            let te = target_segment.1;
            let denominator = (rs.x - re.x) * (ts.y - te.y) - (rs.y - re.y) * (ts.x - te.x);

            if denominator == 0. {
                continue;
            }

            let t = ((rs.x - ts.x) * (ts.y - te.y) - (rs.y - ts.y) * (ts.x - te.x)) / denominator;
            let u = -((rs.x - re.x) * (rs.y - ts.y) - (rs.y - re.y) * (rs.x - ts.x)) / denominator;

            if (0. ..=1.).contains(&t) && (0. ..=1.).contains(&u) {
                colliding_points.push((
                    Vec2 {
                        x: rs.x + t * (re.x - rs.x),
                        y: rs.y + t * (re.y - rs.y),
                    },
                    t,
                ));
            };
        }

        if colliding_points.is_empty() {
            return None;
        };
        Some(
            *colliding_points
                .iter()
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .unwrap(),
        )
    }
}

#[derive(Bundle)]
pub struct RayBundle {
    ray: Ray,
    sprite: SpriteBundle,
}

impl RayBundle {
    pub fn new(ray_length: f32, ray_angle: f32) -> Self {
        Self {
            ray: Ray {
                length: ray_length,
                angle: ray_angle,
                collisions: Vec::new(),
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2 {
                        x: 2.0,
                        y: ray_length,
                    }),
                    anchor: Anchor::BottomCenter,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: 0.,
                        y: 0.,
                        z: -1.,
                    },
                    ..Transform::from_rotation(Quat::from_rotation_z(ray_angle))
                },
                ..Default::default()
            },
        }
    }
}
