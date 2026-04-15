use crate::core::color::Color;
use crate::core::light::Light;
use crate::core::object::{Object, ObjectData};
use crate::core::transform::{Transform, Vec3};
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn in_other_frame(self: &Self, wTc: &Transform) -> Self {
        let cTw = wTc.inverse();
        Self {
            objects: self
                .objects
                .iter()
                .map(|obj| match obj {
                    Object::Sphere(object_data, f) => {
                        Object::Sphere(object_data.transform(&cTw), *f)
                    }
                    Object::Plane(object_data) => Object::Plane(object_data.transform(&cTw)),

                    Object::Box(object_data, f) => Object::Box(object_data.transform(&cTw), *f),
                })
                .collect(),
            lights: vec![Light::Point(Vec3::new(0.5, 0.5, 0.0))],
        }
    }

    pub fn base_scene() -> Self {
        let mut objects = Vec::<Object>::new();
        let t = Transform::at_position(Vec3::new(0.0, 0.0, 1.0));

        objects.push(Object::sphere(t, 0.25));

        {
            let t: Transform = Transform::rotation_around_y(180.0_f64.to_radians())
                .with_new_position(Vec3::new(0.0, 0.0, 1.5));
            objects.push(Object::Plane(ObjectData {
                pose: t.clone(),
                color: Color {
                    r: 0.0,
                    g: 0.8,
                    b: 0.0,
                },
            }));
        }

        {
            let t: Transform = Transform::at_position(Vec3::new(0.0, 1.0, 0.0))
                * Transform::rotation_around_x(-90.0f64.to_radians());
            objects.push(Object::Plane(ObjectData {
                pose: t.clone(),
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                } * 0.8,
            }));
        }

        // let t: Transform = Transform::identity().with_new_position(Vec3::new(0.25, 0.0, 1.0)) ;
        // objects.push(Object::Box(ObjectData {
        //     pose: t,
        //     color: Color {
        //         r: 0.0,
        //         g: 0.0,
        //         b: 1.0,
        //     },
        // }, (0.25, 0.1, 0.1)));
        Scene {
            objects: objects,
            lights: vec![Light::Point(Vec3::new(0.0, 0.0, 0.0))],
        }
    }
}
