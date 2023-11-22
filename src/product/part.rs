use log::{info, warn};
use std::fmt::Display;
use three_d::{
    egui::{Color32, InnerResponse, ProgressBar, Sense, Stroke, Ui},
    Context, Gm, Mesh, Object, PhysicalMaterial,
};
use three_d_asset::{PbrMaterial, Positions, TriMesh, Vector3};

use super::{material::Material, shape::cube, PbrModel};

pub struct Part {
    name: Box<str>,
    shape: TriMesh,
    current_material: usize,
    materials: Box<[Material]>,
    optional: bool,
    opt_in: bool,
    model: Option<PbrModel>,
}
impl Part {
    fn new(name: Box<str>, shape: TriMesh, materials: Box<[Material]>, optional: bool) -> Self {
        Self {
            name,
            shape,
            materials,
            optional,
            current_material: 0,
            opt_in: true,
            model: None,
        }
    }

    pub async fn placeholder_chair() -> Box<[Self]> {
        let asset_info = [
            (
                "./chair/skeleton.obj",
                "Frame",
                Material::placeholder_metals(),
                false,
            ),
            (
                "./chair/plastics.obj",
                "Plastics",
                [Material::black_plastic()].into(),
                false,
            ),
            (
                "./chair/fabrics.obj",
                "Fabrics",
                Material::placeholder_fabs(),
                false,
            ),
            (
                "./chair/plastic_arms.obj",
                "Arm Plastics",
                [Material::black_plastic()].into(),
                true,
            ),
            (
                "./chair/metal_arm.obj",
                "Arm Frame",
                Material::placeholder_metals(),
                true,
            ),
        ];
        let paths: Box<[&str]> = asset_info.iter().map(|row| row.0).collect();
        let mut loaded = if let Ok(loaded) = three_d_asset::io::load_async(&paths).await {
            info!("loaded skybox from assets");
            loaded
        } else {
            panic!("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
        };

        asset_info
            .into_iter()
            .map(|(path, name, materials, optional)| {
                let shape = loaded.deserialize(path).expect("failed to deserialize");
                Self::new(name.into(), shape, materials, optional)
            })
            .collect()
    }

    pub fn placeholder1() -> Self {
        Self {
            name: "Sphere".into(),
            shape: TriMesh::sphere(32),
            current_material: 0,
            materials: Material::placeholder_materials(),
            optional: true,
            opt_in: true,
            model: None,
        }
    }
    pub fn placeholder2() -> Self {
        let shape = cube(0.0, -2.0, 0.);
        Self::new(
            "Cube".into(),
            shape,
            Material::placeholder_materials(),
            false,
        )
    }
    pub fn material(&self) -> &Material {
        &self.materials[self.current_material]
    }
    pub fn shape(&self) -> &TriMesh {
        &self.shape
    }
    pub fn init(&mut self, ctx: &Context) {
        let material = PhysicalMaterial::new_opaque(ctx, &self.material().pbr());
        let mesh = Mesh::new(ctx, &self.shape);
        let model = Gm::new(mesh, material);
        self.model = Some(model);
    }
    pub fn update(&mut self) {
        let rgb = self.material().rgb().into();
        let metallic = self.material().metallic();
        let roughness = self.material().roughness();
        let model = self.model.as_mut();
        match model {
            Some(model) => {
                model.material.albedo = rgb;
                model.material.metallic = metallic;
                model.material.roughness = roughness;
            }
            None => warn!("model has not been initated, doing nothing here!"),
        }
    }

    pub fn object(&self) -> Option<&dyn Object> {
        if self.opt_in {
            Some(self.model.as_ref().expect("model has not been initated"))
        } else {
            None
        }
    }
}

impl Part {
    pub fn add_controls(&mut self, ui: &mut Ui) {
        let options = (self.optional, self.materials.len());

        match options {
            (_, 0) => panic!("Componets must have atleast one material"),
            (false, 1) => (), // self is not configurable
            (true, 1) => self.show_toggle(ui),
            (true, _) => {
                self.show_toggle(ui);
                ui.add_enabled_ui(self.opt_in, |ui| self.material_group(ui));
            }
            (false, _) => {
                ui.label(self.name.as_ref());
                self.material_group(ui);
            }
        }
    }

    fn material_group(&mut self, ui: &mut Ui) -> InnerResponse<()> {
        ui.group(|ui| {
            self.material_picker(ui);
        })
    }
    fn material_picker(&mut self, ui: &mut Ui) {
        for i in 0..self.materials.len() {
            ui.radio_value(&mut self.current_material, i, self.materials[i].to_string());
        }
    }
    fn show_toggle(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.opt_in, self.name.as_ref());
    }
}
