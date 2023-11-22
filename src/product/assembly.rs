use std::{cell::RefCell, rc::Rc};

use log::warn;
use three_d::{egui::Ui, Context, Gm, Mesh, Object, PhysicalMaterial};
use three_d_asset::TriMesh;

use super::{material::Material, shape::cube, PbrModel};

pub enum Include {
    MustHave,
    Optional { opt_in: bool },
}

// impl Include {
//     pub fn new(must_have: bool) -> Self{
//         Self{}

//     }

// }

struct Component {
    shape: TriMesh,
    model: Option<PbrModel>,
}

impl Component {
    fn new(shape: TriMesh) -> Self {
        Self { shape, model: None }
    }

    /// #panics
    /// if self is not initated
    fn object(&self) -> &dyn Object {
        self.model.as_ref().expect("model has not been initated")
    }

    fn init(&mut self, ctx: &Context, material: &Material) {
        let material = PhysicalMaterial::new_opaque(ctx, &material.pbr());
        let mesh = Mesh::new(ctx, &self.shape);
        let model = Gm::new(mesh, material);
        self.model = Some(model);
    }

    fn update(&mut self, material: &Material) {
        let model = self.model.as_mut();
        match model {
            Some(model) => {
                model.material.albedo = material.rgb().into();
                model.material.metallic = material.metallic();
                model.material.roughness = material.roughness();
            }
            None => warn!("model has not been initated, doing nothing here!"),
        }
    }
}

impl From<TriMesh> for Component {
    fn from(shape: TriMesh) -> Self {
        Self::new(shape)
    }
}

type Materials = Box<[Material]>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct MaterialCollection {
    label: Box<str>,
    options: Materials,
    pub current_material: usize,
}

impl MaterialCollection {
    fn new(label: Box<str>, options: Materials) -> Self {
        Self {
            options,
            label,
            current_material: 0,
        }
    }
    fn current(&self) -> &Material {
        &self.options[self.current_material]
    }

    fn len(&self) -> usize {
        self.options.len()
    }
    fn into_shared(self) -> SharedMaterial {
        Rc::new(RefCell::new(self))
    }
}

type MaterialOptions = Box<[MaterialCollection]>;
type IncludeOptions = Box<[Include]>;
struct ComponentMapping {
    component: Component,
    material: usize,
    include: usize,
}
type Components = Box<[ComponentMapping]>;

pub struct Assy {
    components: Components,
    mat_options: MaterialOptions,
    include_options: IncludeOptions,
}

type SharedMaterial = Rc<RefCell<MaterialCollection>>;
// #[derive(PartialEq, Clone, Debug)]
// struct SharedMaterial {
//     material: Rc<RefCell<MaterialCollection>>,
// }

// impl<'a> SharedMaterial {
//     pub fn new(label: Box<str>, materials: Box<[Material]>) -> Self {
//         let collection = MaterialCollection::new(label, materials);
//         let material = Rc::new(RefCell::new(collection));
//         Self { material }
//     }
// }
// impl From<MaterialCollection> for SharedMaterial {
//     fn from(collection: MaterialCollection) -> Self {
//         let material = Rc::new(RefCell::new(collection));
//         Self { material }
//     }
// }
// impl Deref for SharedMaterial {
//     type Target = SharedMaterial;
//     fn deref(&self) -> &Self::Target {
//         &self.material
//     }
// }

// impl DerefMut for SharedMaterial {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.material
//     }
// }

type SharedInclude = Rc<RefCell<Include>>;
pub struct GraphPart {
    name: Box<str>,
    component: Component,
    material: SharedMaterial,
    include: Include,
}

impl GraphPart {
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
    parts: Box<[GraphPart]>,
    materials: Box<[SharedMaterial]>,
    // includes: Box<[SharedInclude]>,
}

impl AssyGraph {
    pub fn dummy() -> Self {
        let metals =
            MaterialCollection::new("metals".into(), Material::placeholder_metals()).into_shared();
        let parts = [
            GraphPart::new(
                "sphere".into(),
                TriMesh::sphere(32).into(),
                metals.clone(),
                Include::MustHave,
            ),
            GraphPart::new(
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

impl<'a> AssyGraph {
    pub fn new(parts: Box<[GraphPart]>) -> Self {
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
            ui.label(material_choice.borrow().label.to_string());
            let n = material_choice.borrow().len();
            for mut i in 0..n {
                let name = material_choice.borrow().options[i].to_string();
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
        self.add_material_ui(ui);
        self.add_configure_ui(ui);
    }
}

impl<'a> Assy {
    pub fn init(&mut self, ctx: &Context) {
        for mapping in self.components.iter_mut() {
            let mat_group = &self.mat_options[mapping.material];
            let material = mat_group.current();
            mapping.component.init(ctx, material)
        }
    }
    fn object(&'a self, mapping: &'a ComponentMapping) -> Option<&'a dyn Object> {
        let part = &mapping.component;
        let index = mapping.include;

        match self.include_options[index] {
            Include::Optional { opt_in: false } => None,
            _ => Some(part.object()),
        }
    }

    /// #panics
    /// if self is not initated
    pub fn objects(&'a self) -> impl Iterator<Item = &'a (dyn three_d::Object + 'a)> {
        self.components
            .iter()
            .filter_map(|mapping| self.object(mapping))
    }
    pub fn new(
        components: Components,
        mat_options: MaterialOptions,
        include_options: IncludeOptions,
    ) -> Self {
        {
            let max_mat = components
                .iter()
                .map(|c| c.material)
                .max()
                .expect("Assy should have atleast one component");
            assert_eq!(
                mat_options.len(),
                max_mat + 1,
                "Components have invalid material mapping",
            );
        }
        let max_include = components
            .iter()
            .map(|c| c.include)
            .max()
            .expect("Assy should have atleast one component");
        assert_eq!(
            include_options.len(),
            max_include + 1,
            "Components have invalid material mapping",
        );
        Self {
            components,
            mat_options,
            include_options,
        }
    }
    // pub fn from_graph(graph: AssyGraph) -> Self {}

    pub fn add_material_ui(&mut self, ui: &mut Ui) {
        for mat_option in self.mat_options.iter_mut() {
            ui.label(mat_option.label.as_ref());
            for i in 0..mat_option.len() {
                ui.radio_value(
                    &mut mat_option.current_material,
                    i,
                    mat_option.options[i].to_string(),
                );
            }
        }
    }
    pub fn update(&mut self) {
        for mapping in self.components.iter_mut() {
            let mat_group = &self.mat_options[mapping.material];
            let material = mat_group.current();
            mapping.component.update(material);
        }
    }
}
