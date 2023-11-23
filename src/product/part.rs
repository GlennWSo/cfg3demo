use log::{info, warn};

use three_d::{
    egui::{InnerResponse, Ui},
    Context, Gm, Mesh, Object, PhysicalMaterial,
};
use three_d_asset::TriMesh;

use super::{
    material::{Material, MaterialCollection},
    shape::cube,
    PbrModel,
};

pub struct Part {
    name: Box<str>,
    shape: TriMesh,
    // current_material: usize,
    // materials: Box<[Material]>,
    material: MaterialCollection,
    optional: bool,
    opt_in: bool,
    model: Option<PbrModel>,
}
impl Part {
    fn new(name: Box<str>, shape: TriMesh, material: MaterialCollection, optional: bool) -> Self {
        Self {
            name,
            shape,
            material,
            optional,
            opt_in: true,
            model: None,
        }
    }

    pub async fn placeholder_chair() -> Box<[Self]> {
        let asset_info = [
            (
                "chair/skeleton.obj",
                "Frame",
                MaterialCollection::metals(),
                false,
            ),
            (
                "chair/plastics.obj",
                "Plastics",
                Material::black_plastic().into(),
                false,
            ),
            (
                "chair/fabrics.obj",
                "Fabrics",
                MaterialCollection::fabrics(),
                false,
            ),
            (
                "chair/plastic_arms.obj",
                "Arm Plastics",
                Material::black_plastic().into(),
                true,
            ),
            (
                "chair/metal_arm.obj",
                "Arm Frame",
                MaterialCollection::metals(),
                true,
            ),
        ];

        #[cfg(target_arch = "wasm32")]
        let paths: Box<[&str]> = asset_info.iter().map(|row| row.0).collect();
        #[cfg(not(target_arch = "wasm32"))]
        let paths: Box<[String]> = asset_info
            .iter()
            .map(|row| format!("./assets/{}", row.0))
            .collect();

        let mut loaded = match three_d_asset::io::load_async(&paths).await {
            Ok(loaded) => {
                info!("loaded skybox from assets");
                loaded
            }
            Err(e) => {
                // log::error!("error: {}", e);
                println!("error: {}", e);
                panic!("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
            }
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
            material: MaterialCollection::metals(),
            optional: true,
            opt_in: true,
            model: None,
        }
    }
    pub fn placeholder2() -> Self {
        let shape = cube(0.0, -2.0, 0.);
        Self::new("Cube".into(), shape, MaterialCollection::metals(), false)
    }
    pub fn material(&self) -> &Material {
        &self.material.current()
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
        let options = (self.optional, self.material.len());

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
        for i in 0..self.material.len() {
            let text = self.material.options()[i].to_string();
            ui.radio_value(&mut self.material.current_material, i, text);
        }
    }
    fn show_toggle(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.opt_in, self.name.as_ref());
    }
}
