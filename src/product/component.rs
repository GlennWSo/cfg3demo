use log::warn;
use std::{
    fmt::Display,
};
use three_d::{
    egui::{Color32, InnerResponse, ProgressBar, Sense, Stroke, Ui},
    Context, Gm, Mesh, Object, PhysicalMaterial,
};
use three_d_asset::{PbrMaterial, TriMesh};

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

type PbrModel = Gm<Mesh, PhysicalMaterial>;

pub struct Component {
    name: Box<str>,
    shape: TriMesh,
    current_material: usize,
    materials: Box<[Material]>,
    optional: bool,
    opt_in: bool,
    model: Option<PbrModel>,
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
            model: None,
        }
    }

    pub fn placeholder() -> Self {
        Self {
            name: "Dummy".into(),
            shape: TriMesh::sphere(32),
            current_material: 0,
            materials: placeholder_materials(),
            optional: true,
            opt_in: true,
            model: None,
        }
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
        let rgb = self.material().rgb.into();
        let metallic = self.material().metallic;
        let roughness = self.material().roughness;
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

impl Component {
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
                self.material_group(ui);
            }
        }
    }

    fn material_group(&mut self, ui: &mut Ui) -> InnerResponse<()> {
        ui.group(|ui| {
            self.material_picker(ui);
            let (response, painter) = ui.allocate_painter([30., 30.].into(), Sense::hover());
            let rounding = 0.;
            let stroke: Stroke = (5.0, Color32::LIGHT_GRAY).into();
            painter.rect(response.rect, rounding, self.material().color32(), stroke);
            ui.horizontal(|ui| {
                ui.label("Metallic:");
                ui.add(ProgressBar::new(self.material().metallic))
            });
            ui.horizontal(|ui| {
                ui.label("Roughness:");
                ui.add(ProgressBar::new(self.material().roughness));
            });
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

// impl<'a> IntoIterator for &'a Component {
//     type Item = &'a dyn Object;

//     type IntoIter = IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.model.into_iter().map(|e| e.into())
//     }
// }
