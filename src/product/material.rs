use std::cell::RefCell;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Deref, DerefMut, Index};
use std::rc::Rc;

use three_d::egui::Color32;
use three_d_asset::PbrMaterial;

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    name: Box<str>,
    rgb: [u8; 3],
    metallic: f32,
    roughness: f32,
}
impl Material {
    fn not_nan(&self) -> bool {
        if self.metallic.is_nan() {
            return false;
        };
        if self.roughness.is_nan() {
            return false;
        };
        true
    }
}

impl Eq for Material {}

impl Hash for Material {
    /// #panics if Nan are pressent in self
    /// TODO create reminder to update hash if Material data structure gets new fields
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        debug_assert!(self.not_nan(), "NaN is not allowed in material sets");
        self.metallic.to_bits().hash(state);
        self.roughness.to_bits().hash(state);
        self.name.hash(state);
        self.rgb.hash(state);
    }
}

impl Material {
    pub fn gold() -> Self {
        Self::new("Gold".into(), [212, 175, 55], 0.9, 0.2)
    }
    pub fn silver() -> Self {
        Self::new("Silver".into(), [192, 192, 192], 0.9, 0.2)
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
    pub fn placeholder_metals() -> Box<[Material]> {
        [Material::alu(), Material::gold(), Material::silver()].into()
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
type Materials = Box<[Material]>;

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct MaterialCollection {
    label: Box<str>,
    options: Materials,
    pub current_material: usize,
}
impl Index<usize> for MaterialCollection {
    type Output = Material;

    fn index(&self, index: usize) -> &Self::Output {
        &self.options[index]
    }
}

impl MaterialCollection {
    pub fn new(label: Box<str>, options: Materials) -> Self {
        Self {
            options,
            label,
            current_material: 0,
        }
    }
    pub fn current(&self) -> &Material {
        &self.options[self.current_material]
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn options(&self) -> &[Material] {
        &self.options
    }

    pub fn len(&self) -> usize {
        self.options.len()
    }
}

/// placholder collections
impl MaterialCollection {
    /// placholder metals
    pub fn metals() -> Self {
        let materials = [Material::alu(), Material::gold(), Material::silver()].into();
        Self::new("Metals".into(), materials)
    }
    /// placholder fabrics
    pub fn fabrics() -> Self {
        let materials = [Material::pink_fabric(), Material::dark_fabric()].into();
        Self::new("Fabrics".into(), materials)
    }
}

pub type SharedMaterialInner = Rc<RefCell<MaterialCollection>>;
#[derive(PartialEq, Clone, Debug)]
pub struct SharedMaterial {
    material: SharedMaterialInner,
}

impl SharedMaterial {
    pub fn mono(material: Material) -> Self {
        MaterialCollection::new(material.name.clone().into(), [material].into()).into()
    }
}

impl From<Material> for SharedMaterial {
    fn from(material: Material) -> Self {
        Self::mono(material)
    }
}

impl From<MaterialCollection> for SharedMaterial {
    fn from(collection: MaterialCollection) -> Self {
        let material = Rc::new(RefCell::new(collection));
        Self { material }
    }
}
impl Deref for SharedMaterial {
    type Target = SharedMaterialInner;
    fn deref(&self) -> &Self::Target {
        &self.material
    }
}

impl DerefMut for SharedMaterial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.material
    }
}
