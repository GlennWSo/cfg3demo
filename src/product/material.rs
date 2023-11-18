use std::fmt::Display;

use three_d::egui::Color32;
use three_d_asset::PbrMaterial;

pub struct Material {
    name: Box<str>,
    rgb: [u8; 3],
    metallic: f32,
    roughness: f32,
}
impl Material {
    fn new(name: &str, rgb: [u8; 3], metallic: f32, roughness: f32) -> Self {
        Self {
            name: name.into(),
            rgb,
            metallic,
            roughness,
        }
    }
    pub fn rgb(&self) -> [u8; 3] {
        self.rgb
    }
    pub fn color32(&self) -> Color32 {
        Color32::from_rgb(self.rgb[0], self.rgb[1], self.rgb[2])
    }
    pub fn pbr(&self) -> PbrMaterial {
        PbrMaterial {
            albedo: self.rgb.into(),
            metallic: self.metallic,
            roughness: self.roughness,
            ..Default::default()
        }
    }
    pub fn placeholder_materials() -> Box<[Material]> {
        [
            Material::new("GreySteel", [132, 132, 132], 0.8, 0.3),
            Material::new("Pink", [213, 114, 207], 0.3, 0.4),
        ]
        .into()
    }

    pub fn metallic(&self) -> f32 {
        self.metallic
    }

    pub(crate) fn roughness(&self) -> f32 {
        self.roughness
    }
}
impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
