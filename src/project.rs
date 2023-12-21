use crate::point_cloud::PointCloud;
pub struct Project {
    project_name: String,
    point_cloud: PointCloud,
}
impl Project {
    pub fn new(project_name: String, point_cloud: PointCloud) -> Project {
        Project {
            project_name,
            point_cloud,
        }
    }
}
