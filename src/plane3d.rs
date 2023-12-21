use crate::color::Color;
use crate::point3d::Point3D;
use crate::vector3d::Vector3D;
/// Plane is a struct that represents a plane in 3D space.
/// The plane is represented by the equation ax + by + cz + d = 0.
#[derive(Debug, Clone, Copy)]
/// 平面を表す構造体
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    color: Color,
}
/// ax + by + cz + d = 0 の形で平面を表現する
impl Plane {
    /// 係数を与えて平面を作成
    pub fn new(a: f32, b: f32, c: f32, d: f32, color: Color) -> Plane {
        Plane {
            a: a,
            b: b,
            c: c,
            d: d,
            color: color,
        }
    }
    /// 色なしで平面を作成
    pub fn new_without_color(a: f32, b: f32, c: f32, d: f32) -> Plane {
        Plane {
            a: a,
            b: b,
            c: c,
            d: d,
            color: [0, 0, 0],
        }
    }
    /// 3点から平面を作成
    pub fn gen_plane_from_points(p1: &Point3D, p2: &Point3D, p3: &Point3D) -> Plane {
        let v1: Vector3D = Vector3D::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let v2: Vector3D = Vector3D::new(p3.x - p1.x, p3.y - p1.y, p3.z - p1.z);
        let normal_vector: Vector3D = v1.cross(&v2);
        let a = normal_vector.x;
        let b = normal_vector.y;
        let c = normal_vector.z;
        let d = -(a * p1.x + b * p1.y + c * p1.z);
        Plane::new_without_color(a, b, c, d)
    }
    pub fn get_normal_vector(&self) -> Vector3D {
        Vector3D::new(self.a, self.b, self.c).normalize()
    }
}
