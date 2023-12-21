use crate::plane3d::Plane;
use pcd_rs::{PcdDeserialize, PcdSerialize};
/// Point3D is a struct that represents a point in 3D space.
#[derive(Debug, Clone, Copy, PartialEq, PcdDeserialize, PcdSerialize)]

pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub intensity: f32,
    pub normal_x: f32,
    pub normal_y: f32,
    pub normal_z: f32,
    pub curvature: f32,
}
impl Point3D {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        intensity: f32,
        normal_x: f32,
        normal_y: f32,
        normal_z: f32,
        curvature: f32,
    ) -> Point3D {
        Point3D {
            x,
            y,
            z,
            intensity,
            normal_x,
            normal_y,
            normal_z,
            curvature,
        }
    }
    /// 点と平面の距離を求める
    pub fn distance_to_plane(&self, plane: &Plane) -> f32 {
        (plane.a * self.x + plane.b * self.y + plane.c * self.z + plane.d).abs()
            / (plane.a * plane.a + plane.b * plane.b + plane.c * plane.c).sqrt()
    }
}
