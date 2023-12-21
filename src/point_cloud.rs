use std::ops::Index;

use crate::point3d::Point3D;
use kdtree::{distance::squared_euclidean, KdTree};

/// PointCloud is a struct that represents a collection of points in 3D space.
#[derive(Debug, Clone)]
/// 点群データを表現する構造体
pub struct PointCloud {
    pub points: Vec<Point3D>,
    pub points_kdtree: KdTree<f32, usize, [f32; 3]>,
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
    pub zmin: f32,
    pub zmax: f32,
}
impl PointCloud {
    /// 点群データの作成
    pub fn new(points: Vec<Point3D>) -> PointCloud {
        let mut points_kdtree: KdTree<f32, usize, [f32; 3]> = KdTree::new(3);
        let mut xmin = points[0].x;
        let mut xmax = points[0].x;
        let mut ymin = points[0].y;
        let mut ymax = points[0].y;
        let mut zmin = points[0].z;
        let mut zmax = points[0].z;
        for i in 0..points.len() {
            points_kdtree
                .add([points[i].x, points[i].y, points[i].z], i)
                .unwrap();
            if points[i].x < xmin {
                xmin = points[i].x;
            }
            if points[i].x > xmax {
                xmax = points[i].x;
            }
            if points[i].y < ymin {
                ymin = points[i].y;
            }
            if points[i].y > ymax {
                ymax = points[i].y;
            }
            if points[i].z < zmin {
                zmin = points[i].z;
            }
            if points[i].z > zmax {
                zmax = points[i].z;
            }
        }
        PointCloud {
            points,
            points_kdtree,
            xmin,
            xmax,
            ymin,
            ymax,
            zmin,
            zmax,
        }
    }
    pub fn boxel_downsampling(&self, boxel_size: f32) -> PointCloud {
        let mut res = vec![];
        let mut progress = 0;
        let xrange = ((self.xmax - self.xmin) / boxel_size) as i64;
        let yrange = ((self.ymax - self.ymin) / boxel_size) as i64;
        let zrange = ((self.zmax - self.zmin) / boxel_size) as i64;
        for x in 0..xrange {
            for y in 0..yrange {
                for z in 0..zrange {
                    if x * y * z * 10 / (xrange * yrange * zrange) >= progress {
                        println!("down sampling {}% done", progress * 10);
                        progress += 1;
                    }
                    let mut points_in_boxel: Vec<Point3D> = vec![];
                    // boxelの中心からboxel_sizeの距離の点を集める
                    for (point, &idx) in self
                        .points_kdtree
                        .within(
                            &[
                                self.xmin + x as f32 * boxel_size + boxel_size / 2.0,
                                self.ymin + y as f32 * boxel_size + boxel_size / 2.0,
                                self.zmin + z as f32 * boxel_size + boxel_size / 2.0,
                            ],
                            boxel_size,
                            &squared_euclidean,
                        )
                        .unwrap()
                    {
                        if self.points[idx].x >= self.xmin + x as f32 * boxel_size
                            && self.points[idx].x < self.xmin + (x + 1) as f32 * boxel_size
                            && self.points[idx].y >= self.ymin + y as f32 * boxel_size
                            && self.points[idx].y < self.ymin + (y + 1) as f32 * boxel_size
                            && self.points[idx].z >= self.zmin + z as f32 * boxel_size
                            && self.points[idx].z < self.zmin + (z + 1) as f32 * boxel_size
                        {
                            points_in_boxel.push(self.points[idx].clone());
                        }
                    }
                    if points_in_boxel.len() > 0 {
                        let mut sum_x = 0.0;
                        let mut sum_y = 0.0;
                        let mut sum_z = 0.0;
                        for point in points_in_boxel.iter() {
                            sum_x += point.x;
                            sum_y += point.y;
                            sum_z += point.z;
                        }
                        // 法線などのデータはもはや意味をなさないので0にする
                        let new_point = Point3D::new(
                            sum_x / points_in_boxel.len() as f32,
                            sum_y / points_in_boxel.len() as f32,
                            sum_z / points_in_boxel.len() as f32,
                            0.0,
                            0.0,
                            0.0,
                            0.0,
                            0.0,
                        );
                        res.push(new_point);
                    }
                }
            }
        }
        PointCloud::new(res)
    }
    pub fn size(&self) -> usize {
        return self.points.len();
    }
}
pub type Id = usize;

impl Index<Id> for PointCloud {
    type Output = Point3D;
    /// indexedアクセスできるように
    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}
