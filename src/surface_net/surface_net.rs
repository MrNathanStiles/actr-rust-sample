// import { actr_log } from "./log";
// import { ActrPoint3 } from "./point";
// import { BufferGeometry } from "./three-geometry";
// import { MeshStandardMaterial } from "./three-material";
// import { Mesh } from "./three-mesh";
// import { Scene } from "./three-scene";

use crate::{
    actr::{
        actr_three_geometry_buffer, actr_three_geometry_dispose, actr_three_material_dispose,
        actr_three_material_standard, actr_three_mesh,
    },
    component::vector3::Vector3,
};

pub struct SurfaceNet {
    pub geometry: i32,
    pub material: i32,
    pub mesh: i32,
    pub disposed: bool,
    pub size: Vector3,
}

impl SurfaceNet {
    pub fn new(vertices: Vec<f32>, indices: Vec<usize>, size: Vector3) -> SurfaceNet {
        unsafe {
            let geometry = actr_three_geometry_buffer(
                indices.len(),
                indices.as_ptr() as usize,
                vertices.len(),
                vertices.as_ptr() as usize,
            );
            let material =
                actr_three_material_standard(0xffffff, 0xffffff, false, 1.0, true, true);
            let mesh = actr_three_mesh(geometry, material);
            SurfaceNet {
                geometry,
                material,
                mesh,
                disposed: false,
                size,
            }
        }
    }

    pub fn dispose(&mut self) {
        if self.disposed {
            return;
        }
        unsafe {
            actr_three_geometry_dispose(self.geometry);
            actr_three_material_dispose(self.material);
        }
        self.disposed = true;
    }
}
