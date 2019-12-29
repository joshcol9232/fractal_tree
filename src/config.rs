use yaml_rust::Yaml;

pub struct Config {
    pub start_angle: f32,
    pub angular_velocity: f32,
    pub iterations: usize,
    pub branches_per_iteration: usize,
    pub line_thickness: f32,
}

impl From<&Yaml> for Config {
    fn from(conf: &Yaml) -> Self {
        Self {
            start_angle: conf["start_angle"].as_f64()
                .expect("Could not parse start_angle as a float.") as f32,
            angular_velocity: conf["angular_velocity"].as_f64()
                .expect("Could not parse angular_velocity as a float.") as f32,
            iterations: conf["iterations"].as_i64()
                .expect("Could not parse iterations as an integer.") as usize,
            branches_per_iteration: conf["branches_per_iteration"].as_i64()
                .expect("Could not parse branches_per_iteration as an integer.") as usize,
            line_thickness: conf["line_thickness"].as_f64()
                .expect("Could not parse line_thickness as a float.") as f32,
        }
    }
}