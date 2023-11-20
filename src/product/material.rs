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
    pub fn gold() -> Self {
        Self::new("Gold".into(), [212, 175, 55], 1.0, 0.1)
    }
    pub fn silver() -> Self {
        Self::new("Silver".into(), [192, 192, 192], 1.0, 0.1)
    }
    pub fn alu() -> Self {
        Self::new("Aluminium".into(), [132, 135, 137], 0.7, 0.3)
    }
    pub fn black_plastic() -> Self {
        Self::new("Black Plastic".into(), [34, 35, 39], 0.1, 0.4)
    }
    pub fn dark_fabric() -> Self {
        Self::new("Dark Fabric".into(), [20, 39, 46], 0.3, 0.9)
    }
    pub fn pink_fabric() -> Self {
        Self::new("Pink Fabric".into(), [255, 138, 201], 0.3, 0.9)
    }

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

    pub fn placeholder_fabs() -> Box<[Material]> {
        [Material::dark_fabric(), Material::pink_fabric()].into()
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
