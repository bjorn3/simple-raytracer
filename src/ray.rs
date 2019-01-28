use super::light::Light;
use super::material::Color;
use super::material::Material;
use super::shapes::Shapes;
use super::vector::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Debug)]
pub struct Intersection {
    pub distance: f64,
    pub hit_point: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

impl Ray {
    pub fn intersect(ray: &Ray, objects: &Vec<Shapes>) -> Option<Intersection> {
        let mut distance = std::f64::INFINITY;
        let mut material = Material {
            diffuse_color: Color::new(0.0, 0.0, 0.0),
            specular_exponent: 0.0,
        };
        let mut normal = Vector3::zero();
        let mut hit = Vector3::zero();

        for shape in objects {
            match shape.intersect(&ray) {
                Option::Some(dist) => {
                    if dist < distance {
                        distance = dist;
                        material = shape.material();

                        hit = ray.origin + (ray.direction * distance);
                        normal = shape.normal(hit);
                    }
                }
                Option::None => (),
            }
        }

        if distance < std::f64::INFINITY {
            Option::Some(Intersection {
                distance: distance,
                hit_point: hit,
                normal: normal,
                material: material,
            })
        } else {
            Option::None
        }
    }

    pub fn cast_ray(ray: &Ray, objects: &Vec<Shapes>, lights: &Vec<Light>) -> Option<Color> {
        match Ray::intersect(ray, objects) {
            Option::Some(intersection) => {
                let light_int =
                    Light::calculate_intensity(objects, lights, &intersection, ray.direction);

                let color = intersection.material.diffuse_color * light_int;

                Option::Some(color)
            }
            Option::None => Option::None,
        }
    }
}
