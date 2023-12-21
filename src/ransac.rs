use crate::plane3d::Plane;
use crate::point3d::Point3D;
use crate::point_cloud::PointCloud;
use rand::Rng;

pub fn ransac(point_cloud: PointCloud, threshold: f32, iterations: usize) -> (Plane, Vec<Point3D>) {
    let mut best_plane: Plane = Plane::new_without_color(0.0, 0.0, 0.0, 0.0);
    let mut point_in_best_plane: Vec<Point3D> = vec![];
    let mut best_plane_size: usize = 0;
    let mut progress: usize = 0;
    for _ in 0..iterations {
        if iterations >= progress * iterations / 10 {
            println!("{}% done", progress * 10);
            progress += 1;
        }
        // 平面に属する点を集める
        let mut point_in_plane: Vec<Point3D> = vec![];
        let mut plane_size: usize = 0;
        let mut used_points: Vec<Point3D> = Vec::new();
        let mut unused_points: Vec<Point3D> = Vec::new();
        let mut rng = rand::thread_rng();
        let mut random_points = vec![];
        // 3点をランダムに選んで平面を作成
        for _ in 0..3 {
            let random_point: Point3D =
                point_cloud.points[rng.gen_range(0..point_cloud.points.len())].clone();
            random_points.push(random_point.clone());
            used_points.push(random_point.clone());
        }
        let plane: Plane =
            Plane::gen_plane_from_points(&random_points[0], &random_points[1], &random_points[2]);
        // 平面に属する点を集める
        for point in point_cloud.points.iter() {
            if !used_points.contains(point) {
                let distance = point.distance_to_plane(&plane);
                if distance < threshold {
                    point_in_plane.push(point.clone());
                    used_points.push(point.clone());
                    plane_size += 1;
                } else {
                    unused_points.push(point.clone());
                }
            }
        }
        // 平面に属する点が多ければ更新
        if plane_size > best_plane_size {
            point_in_best_plane = point_in_plane;
            best_plane = plane;
            best_plane_size = plane_size;
        }
    }
    (best_plane, point_in_best_plane)
}
