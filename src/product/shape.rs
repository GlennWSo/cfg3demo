use three_d_asset::{Positions, TriMesh};

pub fn cube(dx: f32, dy: f32, dz: f32) -> TriMesh {
    let mut shape = TriMesh::cube();
    let pos = match shape.positions {
        Positions::F32(mut v) => Positions::F32({
            for e in v.iter_mut() {
                e.x += dx;
                e.y += dy;
                e.z += dz;
            }
            v
        }),
        Positions::F64(mut v) => Positions::F64({
            for e in v.iter_mut() {
                e.x += dx as f64;
                e.y += dy as f64;
                e.z += dz as f64;
            }
            v
        }),
    };

    shape.positions = pos;
    shape
}
