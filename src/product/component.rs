use std::fmt::Display;
use three_d::egui;
use three_d_asset::TriMesh;

use egui::Ui;

pub struct Material {
    pub name: Box<str>,
    pub rgb: [u8; 3],
    pub metallic: f32,
    pub roughness: f32,
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
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn placeholder_materials() -> Box<[Material]> {
    [
        Material::new("GreySteel", [132, 132, 132], 0.8, 0.3),
        Material::new("Pink", [213, 114, 207], 0.3, 0.4),
    ]
    .into()
}

pub struct Component {
    name: Box<str>,
    shape: TriMesh,
    current_material: usize,
    materials: Box<[Material]>,
    optional: bool,
    opt_in: bool,
}
impl Component {
    fn new(name: Box<str>, shape: TriMesh, materials: Box<[Material]>, optional: bool) -> Self {
        Self {
            name,
            shape,
            materials,
            optional,
            current_material: 0,
            opt_in: true,
        }
    }

    pub fn placeholder() -> Self {
        Self {
            name: "Dummy".into(),
            shape: TriMesh::sphere(32),
            current_material: 0,
            materials: placeholder_materials(),
            optional: true,
            opt_in: false,
        }
    }
    pub fn material(&self) -> &Material {
        &self.materials[self.current_material]
    }
}

impl Component {
    pub fn material_picker(&mut self, ui: &mut Ui, rgb: &mut [u8; 3]) {
        for i in 0..self.materials.len() {
            ui.radio_value(&mut self.current_material, i, self.materials[i].to_string());
        }
        *rgb = self.material().rgb;
    }
    fn show_toggle(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.opt_in, self.name.as_ref());
    }
}
