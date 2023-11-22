use std::cell::RefCell;

use three_d::{egui::Ui, Context};
use three_d_asset::TriMesh;

use super::{
    component::Component,
    material::{self, Material, MaterialCollection, SharedMaterial},
    shape::cube,
};

pub enum Include {
    MustHave,
    Optional { opt_in: bool },
}

pub struct ConfigPart {
    name: Box<str>,
    component: Component,
    material: SharedMaterial,
    include: Include,
}
impl From<(&str, Component, SharedMaterial, Include)> for ConfigPart {
    fn from(value: (&str, Component, SharedMaterial, Include)) -> Self {
        let (name, component, material, include) = value;
        Self::new(name.into(), component, material, include)
    }
}

impl ConfigPart {
    pub fn new(
        name: Box<str>,
        component: Component,
        material: SharedMaterial,
        include: Include,
    ) -> Self {
        Self {
            name,
            component,
            material,
            include,
        }
    }
}

pub struct AssyGraph {
    parts: Box<[ConfigPart]>,
    materials: Box<[SharedMaterial]>,
    // includes: Box<[SharedInclude]>,
}

impl<'a> AssyGraph {
    pub fn new(parts: Box<[ConfigPart]>) -> Self {
        let mut materials = Vec::new();
        for p in parts.iter() {
            if !materials.contains(&p.material) {
                materials.push(p.material.clone());
            }
        }
        Self {
            parts,
            materials: materials.into(),
        }
    }

    pub fn init(&mut self, ctx: &Context) {
        for p in self.parts.iter_mut() {
            p.component.init(ctx, p.material.borrow().current())
        }
    }
    pub fn objects(&'a self) -> impl Iterator<Item = &'a (dyn three_d::Object + 'a)> {
        self.parts.iter().filter_map(|part| match part.include {
            Include::Optional { opt_in: false } => None,
            _ => Some(part.component.object()),
        })
    }
    pub fn update(&mut self) {
        for part in self.parts.iter_mut() {
            part.component.update(part.material.borrow().current());
        }
    }
    pub fn add_material_ui(&mut self, ui: &mut Ui) {
        for material_choice in self.materials.iter_mut() {
            ui.label(material_choice.borrow().label().to_string());
            let n = material_choice.borrow().len();
            for i in 0..n {
                let name = material_choice.borrow()[i].to_string();
                ui.radio_value(
                    &mut RefCell::borrow_mut(material_choice).current_material,
                    i,
                    name,
                );
            }
        }
    }
    pub fn add_configure_ui(&mut self, ui: &mut Ui) {
        for part in self.parts.iter_mut() {
            match &mut part.include {
                Include::MustHave => continue,
                Include::Optional { opt_in } => {
                    ui.checkbox(opt_in, part.name.as_ref());
                }
            }
        }
    }
    pub fn add_controls(&mut self, ui: &mut Ui) {
        self.add_configure_ui(ui);
        self.add_material_ui(ui);
    }
}
/// placeholders
impl AssyGraph {
    pub async fn placeholder_chair() -> Self {
        let metals: SharedMaterial = MaterialCollection::metals().into();
        let fabs: SharedMaterial = MaterialCollection::fabrics().into();
        let plastic: SharedMaterial = Material::black_plastic().into();
        let shapes = Component::placeholder_chair().await;
        let materials = [
            plastic,
            metals.clone(),
            metals.clone(),
            fabs.clone(),
            fabs.clone(),
        ];
        let includes = [
            Include::MustHave,
            Include::MustHave,
            Include::Optional { opt_in: true },
            Include::MustHave,
            Include::Optional { opt_in: true },
        ];

        let data = shapes
            .into_iter()
            .zip(materials)
            .zip(includes)
            .map(|(((name, comp), material), inc)| (name, comp, material, inc).into())
            .collect();

        Self::new(data)
    }
    pub fn dummy() -> Self {
        let metals: SharedMaterial = MaterialCollection::metals().into();
        let parts = [
            ConfigPart::new(
                "sphere".into(),
                TriMesh::sphere(32).into(),
                metals.clone(),
                Include::MustHave,
            ),
            ConfigPart::new(
                "cube".into(),
                cube(0.0, -2.0, 0.0).into(),
                metals.clone(),
                Include::Optional { opt_in: true },
            ),
        ]
        .into();
        Self::new(parts)
    }
}
