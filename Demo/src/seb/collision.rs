


use crate::seb::primitives::{Rectangle, Sphere};
use nalgebra_glm as glm;


pub enum Collider {
    Rectangle(Rectangle),
    Sphere(Sphere),
}
impl From<Rectangle> for Collider {
    fn from(value: Rectangle) -> Self {
        Collider::Rectangle(value)
    }
}
impl From<Sphere> for Collider {
    fn from(value: Sphere) -> Self {
        Collider::Sphere(value)
    }
}


/*
ATENTIE pt rezolvarea coliziunii:
ADUNI LA PRIMUL SCAZI LA AL DOILEA LA MTV
*/
pub fn collide(this: Collider, other: Collider) -> Option<glm::Vec3> {
    match (&this, &other) {
        (Collider::Sphere(a), Collider::Sphere(b)) => {
            let delta = b.position - a.position;
            let dist = glm::length(&delta);
            let total_radius = a.scale + b.scale;

            if dist >= total_radius {
                return None;
            }

            let direction = if dist > 1e-5 {
                delta / dist
            } else {
                glm::vec3(1.0, 0.0, 0.0)
            };

            let overlap = total_radius - dist;
            Some(-direction * overlap)
        }

        // Rectangle vs Rectangle
        (Collider::Rectangle(a), Collider::Rectangle(b)) => {
            let model_a = a.get_model();
            let model_b = b.get_model();

            let a_axes = [
                glm::normalize(&glm::column(&model_a, 0).xyz()),
                glm::normalize(&glm::column(&model_a, 1).xyz()),
                glm::normalize(&glm::column(&model_a, 2).xyz()),
            ];

            let b_axes = [
                glm::normalize(&glm::column(&model_b, 0).xyz()),
                glm::normalize(&glm::column(&model_b, 1).xyz()),
                glm::normalize(&glm::column(&model_b, 2).xyz()),
            ];

            let mut axes = Vec::with_capacity(15);
            axes.extend_from_slice(&a_axes);
            axes.extend_from_slice(&b_axes);

            for &a_axis in &a_axes {
                for &b_axis in &b_axes {
                    let cross = glm::cross(&a_axis, &b_axis);
                    if glm::length(&cross) > 1e-5 {
                        axes.push(glm::normalize(&cross));
                    }
                }
            }

            let a_center = glm::column(&model_a, 3).xyz();
            let b_center = glm::column(&model_b, 3).xyz();
            let center_diff = b_center - a_center;

            let mut min_overlap = f32::MAX;
            let mut mtv_axis = glm::vec3(0.0, 0.0, 0.0);

            for axis in axes {
                let a_proj = {
                    let r = (glm::dot(&a_axes[0], &axis).abs() * a.scale.x)
                        + (glm::dot(&a_axes[1], &axis).abs() * a.scale.y)
                        + (glm::dot(&a_axes[2], &axis).abs() * a.scale.z);
                    let center_proj = glm::dot(&a_center, &axis);
                    (center_proj - r, center_proj + r)
                };

                let b_proj = {
                    let r = (glm::dot(&b_axes[0], &axis).abs() * b.scale.x)
                        + (glm::dot(&b_axes[1], &axis).abs() * b.scale.y)
                        + (glm::dot(&b_axes[2], &axis).abs() * b.scale.z);
                    let center_proj = glm::dot(&b_center, &axis);
                    (center_proj - r, center_proj + r)
                };

                let overlap = f32::min(a_proj.1, b_proj.1) - f32::max(a_proj.0, b_proj.0);
                if overlap < 0.0 {
                    return None;
                }

                if overlap < min_overlap {
                    min_overlap = overlap;
                    mtv_axis = axis;

                    if glm::dot(&center_diff, &mtv_axis) < 0.0 {
                        mtv_axis = -mtv_axis;
                    }
                }
            }

            Some(-mtv_axis * min_overlap)
        }

        // Sphere vs Rectangle (și invers)
        (Collider::Sphere(sphere), Collider::Rectangle(rect)) => {
            let model = rect.get_model();

            let rect_center = rect.position;

            // Extrage axele și normalizează-le (ca să elimini efectul scalei)
            let axis_x = glm::normalize(&glm::vec3(model[(0, 0)], model[(1, 0)], model[(2, 0)]));
            let axis_y = glm::normalize(&glm::vec3(model[(0, 1)], model[(1, 1)], model[(2, 1)]));
            let axis_z = glm::normalize(&glm::vec3(model[(0, 2)], model[(1, 2)], model[(2, 2)]));

            let half_scale = rect.scale;
            let difference = sphere.position - rect_center;

            let mut closest_point = rect_center;
            for (axis, half_extent) in [
                (axis_x, half_scale.x),
                (axis_y, half_scale.y),
                (axis_z, half_scale.z),
            ] {
                let distance = glm::dot(&difference, &axis);
                let clamped = distance.clamp(-half_extent, half_extent);
                closest_point += clamped * axis;
            }

            let to_sphere = sphere.position - closest_point;
            let dist_sq = glm::dot(&to_sphere, &to_sphere);

            let radius = sphere.scale;

            if dist_sq < sphere.scale * sphere.scale {
                let dist = dist_sq.sqrt();
                // Dacă distanța e aproape 0, generăm un normal artificial
                let normal = if dist != 0.0 {
                    to_sphere / dist
                } else {
                    glm::vec3(1.0, 0.0, 0.0) // fallback
                };
                let penetration = radius - dist;
                let mtv = normal * penetration;
                Some(mtv)
            } else {
                None
            }
        }
        (Collider::Rectangle(rect), Collider::Sphere(sphere)) => {
            let model = rect.get_model();
            let rect_center = rect.position;

            // Normalizează axele (elimină scala)
            let axis_x = glm::normalize(&glm::vec3(model[(0, 0)], model[(1, 0)], model[(2, 0)]));
            let axis_y = glm::normalize(&glm::vec3(model[(0, 1)], model[(1, 1)], model[(2, 1)]));
            let axis_z = glm::normalize(&glm::vec3(model[(0, 2)], model[(1, 2)], model[(2, 2)]));

            let half_scale = rect.scale;
            let difference = sphere.position - rect_center;

            // Găsește cel mai apropiat punct de pe OBB față de centrul sferei
            let mut closest_point = rect_center;
            for (axis, half_extent) in [
                (axis_x, half_scale.x),
                (axis_y, half_scale.y),
                (axis_z, half_scale.z),
            ] {
                let distance = glm::dot(&difference, &axis);
                let clamped = distance.clamp(-half_extent, half_extent);
                closest_point += clamped * axis;
            }

            // Verifică dacă acel punct este în interiorul sferei
            let to_closest = closest_point - sphere.position;
            let dist_sq = glm::dot(&to_closest, &to_closest);

            let radius = sphere.scale;

            if dist_sq < radius * radius {
                let dist = dist_sq.sqrt();
                let normal = if dist != 0.0 {
                    to_closest / dist
                } else {
                    glm::vec3(0.0, 1.0, 0.0) // fallback
                };
                let penetration = radius - dist;
                let mtv = normal * penetration;
                Some(mtv)
            } else {
                None
            }
        }
    }
}
