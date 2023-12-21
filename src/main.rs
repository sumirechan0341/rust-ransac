mod color;
mod plane3d;
mod point3d;
mod point_cloud;
mod project;
mod ransac;
mod vector3d;
use pcd_rs::{anyhow::Error, DataKind, Reader, WriterInit};
use point3d::Point3D;
use std::env;
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let reader: Reader<Point3D, std::io::BufReader<std::fs::File>> =
        Reader::open(&args[1]).unwrap();
    let points = reader
        .filter_map(|p| {
            if let Ok(y) = p {
                return Some(y);
            } else {
                return None;
            }
        })
        .collect::<Vec<Point3D>>();
    let point_cloud = point_cloud::PointCloud::new(points);
    let downsampled_point_cloud = point_cloud.boxel_downsampling(0.1);
    println!("original {}", point_cloud.size());
    println!("down sampled {}", downsampled_point_cloud.size());
    let (plane, point_in_plane) = ransac::ransac(downsampled_point_cloud, 0.001, 1000);
    // serialize points
    let mut writer = WriterInit {
        width: point_in_plane.len() as u64,
        height: 1,
        viewpoint: Default::default(),
        data_kind: DataKind::Ascii,
        schema: None,
    }
    .create("./ransac.pcd")?;
    for point in point_in_plane {
        writer.push(&point)?;
    }

    writer.finish()?;
    Ok(())
}
