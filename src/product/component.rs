use log::{info, warn};
use three_d::{Context, Gm, Mesh, Object, PhysicalMaterial};
use three_d_asset::{AxisAlignedBoundingBox as AABB, TriMesh, Vector3};

use super::{material::Material, PbrModel};

pub struct Body {
    shape: TriMesh,
    model: Option<PbrModel>,
}

impl Body {
    fn new(shape: TriMesh) -> Self {
        Self { shape, model: None }
    }

    pub fn bounding_box(&self) -> AABB {
        self.shape.compute_aabb()
    }

    pub fn bb_center(&self) -> Vector3<f32> {
        self.shape.compute_aabb().center()
    }

    /// #panics
    /// if self is not initated
    pub fn object(&self) -> &dyn Object {
        self.model.as_ref().expect("model has not been initated")
    }

    pub fn init(&mut self, ctx: &Context, material: &Material) {
        let material = PhysicalMaterial::new_opaque(ctx, &material.pbr());
        let mesh = Mesh::new(ctx, &self.shape);
        let model = Gm::new(mesh, material);
        self.model = Some(model);
    }

    pub fn update(&mut self, material: &Material) {
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

impl From<TriMesh> for Body {
    fn from(shape: TriMesh) -> Self {
        Self::new(shape)
    }
}

impl Body {
    pub async fn placeholder_chair() -> Vec<(&'static str, Self)> {
        let info = [
            ("Platic Parts", "chair/plastics.obj"),
            ("Base Frame", "chair/skeleton.obj"),
            ("Arm Frame", "chair/metal_arm.obj"),
            ("Fabrics", "chair/fabrics.obj"),
            ("Arm Fabrics", "chair/plastic_arms.obj"),
        ];
        let paths: Vec<_> = info.iter().map(|row| row.1).collect();
        #[cfg(not(target_arch = "wasm32"))]
        let paths: Vec<_> = paths
            .into_iter()
            .map(|p| format!("./assets/{}", p))
            .collect();
        let mut loaded = if let Ok(loaded) = three_d_asset::io::load_async(&paths).await {
            info!("loaded skybox from assets");
            loaded
        } else {
            panic!("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
        };

        info.map(|(name, path)| {
            let shape = loaded.deserialize(path).expect("failed to deserialize");
            (name, Self::new(shape))
        })
        .into()
    }
}
