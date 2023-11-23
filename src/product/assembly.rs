use std::{cell::RefCell, ops::Deref, rc::Rc};

use three_d::{egui::Ui, Context};
use three_d_asset::{AxisAlignedBoundingBox as AABB, TriMesh, Vector3};

use super::{
    component::Body,
    material::{Material, MaterialCollection, SharedMaterial},
    shape::cube,
};

type SharedToggle = Rc<RefCell<bool>>;

#[derive(Clone, PartialEq, Eq)]
pub enum Include {
    MustHave,
    Optional {
        label: Box<str>,
        opt_in: SharedToggle,
    },
}
impl Include {
    fn optinal(name: impl Into<Box<str>>, value: bool) -> Self {
        Include::Optional {
            label: name.into(),
            opt_in: Rc::new(RefCell::new(value)),
        }
    }
    fn get_toggle(&self) -> Option<(&str, &SharedToggle)> {
        match self {
            Include::Optional { label, opt_in } => Some((label, opt_in)),
            _ => None,
        }
    }
    fn is_show(&self) -> bool {
        match self {
            Include::Optional { opt_in, .. } => *opt_in.borrow(),
            _ => true,
        }
    }
}

pub struct ConfigPart {
    #[allow(dead_code)]
    name: Box<str>,
    body: Body,
    material: SharedMaterial,
    include: Include,
}
impl From<(&str, Body, SharedMaterial, Include)> for ConfigPart {
    fn from(value: (&str, Body, SharedMaterial, Include)) -> Self {
        let (name, component, material, include) = value;
        Self::new(name.into(), component, material, include)
    }
}

impl ConfigPart {
    pub fn new(
        name: Box<str>,
        component: Body,
        material: SharedMaterial,
        include: Include,
    ) -> Self {
        Self {
            name,
            body: component,
            material,
            include,
        }
    }
}

pub struct Assy {
    parts: Box<[ConfigPart]>,
    materials: Box<[SharedMaterial]>,
    includes: Box<[Include]>,
}

impl<'a> Assy {
    pub fn new(parts: Box<[ConfigPart]>) -> Self {
        assert!(parts.len() > 0, "Assy must have atleast one part");
        let mut materials = Vec::new();
        let mut includes = Vec::new();
        for p in parts.iter() {
            if !materials.contains(&p.material) {
                materials.push(p.material.clone());
            }
            if !includes.contains(&p.include) {
                includes.push(p.include.clone());
            }
        }
        Self {
            parts,
            materials: materials.into(),
            includes: includes.into(),
        }
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }

    pub fn bbox(&self) -> AABB {
        let mut bb = self.parts[0].body.bounding_box();
        if self.len() == 1 {
            bb
        } else {
            let boxes = self.parts.iter().map(|p| p.body.bounding_box());
            for other_box in boxes {
                bb.expand_with_aabb(&other_box);
            }
            bb
        }
    }

    pub fn init(&mut self, ctx: &Context) {
        for p in self.parts.iter_mut() {
            p.body.init(ctx, p.material.borrow().current())
        }
    }
    pub fn objects(&'a self) -> impl Iterator<Item = &'a (dyn three_d::Object + 'a)> {
        self.parts.iter().filter_map(|part| {
            if part.include.is_show() {
                Some(part.body.object())
            } else {
                None
            }
        })
    }
    pub fn update(&mut self) {
        for part in self.parts.iter_mut() {
            part.body.update(part.material.borrow().current());
        }
    }
    pub fn add_material_ui(&mut self, ui: &mut Ui) {
        for material_choice in self.materials.iter().filter(|m| m.borrow().len() > 1) {
            ui.add_space(10.0);
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
        ui.add_space(10.);
        for (name, toggle) in self.includes.iter().filter_map(|inc| inc.get_toggle()) {
            ui.checkbox(&mut toggle.borrow_mut(), name);
        }
    }
    pub fn add_controls(&mut self, ui: &mut Ui) {
        self.add_configure_ui(ui);
        self.add_material_ui(ui);
    }
}
/// placeholders
impl Assy {
    pub async fn placeholder_chair() -> Self {
        let metals: SharedMaterial = MaterialCollection::metals().into();
        let fabs: SharedMaterial = MaterialCollection::fabrics().into();
        let plastic: SharedMaterial = Material::black_plastic().into();
        let shapes = Body::placeholder_chair().await;
        let materials = [
            plastic,
            metals.clone(),
            metals.clone(),
            fabs.clone(),
            fabs.clone(),
        ];
        let arm_option = Include::optinal("Arms", true);
        let includes = [
            Include::MustHave,
            Include::MustHave,
            arm_option.clone(),
            Include::MustHave,
            arm_option,
        ];

        let data = shapes
            .into_iter()
            .zip(materials)
            .zip(includes)
            .map(|(((name, comp), material), inc)| (name, comp, material, inc).into())
            .collect();

        Self::new(data)
    }
    #[allow(dead_code)]
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
                Include::optinal("Cube", true),
            ),
        ]
        .into();
        Self::new(parts)
    }
}
