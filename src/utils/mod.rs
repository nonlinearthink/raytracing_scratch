mod ppm;

pub use ppm::*;

pub fn deg_to_rad(degree: f32) -> f32 {
    degree * std::f32::consts::PI / 180.0
}

pub fn rad_to_deg(radian: f32) -> f32 {
    radian * 180. / std::f32::consts::PI
}

pub fn linear_to_gramma(linear_component: f32) -> f32 {
    f32::sqrt(linear_component)
}

pub fn gramma_to_linear(gramma_component: f32) -> f32 {
    gramma_component.powi(2)
}
