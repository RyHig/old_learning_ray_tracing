use ray_lib::Ray;
use vec_lib::Vector3;

struct hit_record {
    t: f64,
    p: Vector3,
    normal: Vector3,
}

pub struct hitable {
    r: Ray,
    t_min: f64,
    t_max: f64,
    rec: hit_record,
}

impl hitable{
    pub fn hit(self) -> bool {
        
    }
}