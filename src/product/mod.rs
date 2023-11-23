mod assembly;
mod component;
pub mod material;
mod part;
mod shape;

pub use part::Part;
use three_d::{egui::Ui, AxisAlignedBoundingBox as AABB, Context, Gm, Mesh, PhysicalMaterial};

use assembly::Assy;

pub type PbrModel = Gm<Mesh, PhysicalMaterial>;

pub struct Product {
    name: Box<str>,
    parts: Box<[Part]>,
    assys: Box<[Assy]>,
}

impl<'a> Product {
    fn new(name: &str, parts: Box<[Part]>, assys: Box<[Assy]>) -> Self {
        Self {
            name: name.into(),
            parts,
            assys,
        }
    }

    pub fn init(&mut self, ctx: &Context) {
        for part in self.parts.iter_mut() {
            part.init(ctx);
        }
        for assy in self.assys.iter_mut() {
            assy.init(ctx);
        }
    }

    #[allow(dead_code)]
    pub async fn placeholder() -> Self {
        // let parts = [Component::placeholder1(), Component::placeholder2()].into();
        let parts = Part::placeholder_chair().await;
        Self::new("Chair (tm)", parts, [].into())
    }

    pub async fn assy_dummy() -> Self {
        let assy = Assy::placeholder_chair().await;
        Self::new("Dummy", [].into(), [assy].into())
    }

    pub fn objects(&'a self) -> impl Iterator<Item = &'a (dyn three_d::Object + 'a)> {
        let assy_objects = self.assys.iter().flat_map(|assy| assy.objects());
        let part_objects = self.parts.iter().filter_map(move |part| part.object());
        assy_objects.chain(part_objects)
    }

    pub fn add_controls(&mut self, ui: &mut Ui) {
        ui.heading(self.name.as_ref());
        for part in self.parts.iter_mut() {
            ui.add_space(10.0);
            part.add_controls(ui)
        }
        for assy in self.assys.iter_mut() {
            assy.add_controls(ui);
        }
    }

    pub fn update(&mut self) {
        for part in self.parts.iter_mut() {
            part.update();
        }
        for assy in self.assys.iter_mut() {
            assy.update();
        }
    }

    fn parts_bb(&self) -> Option<AABB> {
        let len = self.parts.len();
        if len == 0 {
            return None;
        }
        let mut bb = self.parts[0].shape().compute_aabb();
        if len == 1 {
            return Some(bb);
        }
        for other_bb in self.parts[1..].iter().map(|p| p.shape().compute_aabb()) {
            bb.expand_with_aabb(&other_bb);
        }
        Some(bb)
    }
    fn assys_bb(&self) -> Option<AABB> {
        let len = self.assys.len();
        if len == 0 {
            return None;
        }
        let mut bb = self.assys[0].bbox();
        if len == 1 {
            return Some(bb);
        }
        for other_bb in self.assys[1..].iter().map(|a| a.bbox()) {
            bb.expand_with_aabb(&other_bb);
        }
        Some(bb)
    }
    pub fn bbox(&self) -> AABB {
        let bb1 = self.parts_bb();
        let bb2 = self.assys_bb();
        match (bb1, bb2) {
            (None, None) => panic!("product has no parts nor assemblies"),
            (None, Some(bb)) | (Some(bb), None) => bb,
            (Some(mut bb), Some(bb2)) => {
                bb.expand_with_aabb(&bb2);
                bb
            }
        }
    }
}
