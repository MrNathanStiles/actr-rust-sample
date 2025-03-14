// import { SurfaceNetData } from "./surface-net-data";
// import { PointDataGen as SurfaceNetPointgenerator, precomputeEdgeTable } from "./surface-nets";

use super::{PointDataGen, surface_net_data::SurfaceNetData};

pub struct SurfaceNetGenerator {}
//public constructor() {
//  precomputeEdgeTable();
//}

impl SurfaceNetGenerator {
    pub fn make_data(dims: Vec<Vec<f32>>, f: PointDataGen) -> SurfaceNetData {
        let mut res: Vec<usize> = Vec::with_capacity(3);
        for i in 0..3 {
            let value = 2.0 + (dims[i][1] - dims[i][0]).ceil() / dims[i][2];
            res.push(value as usize);
        }
        let mut volume = Vec::<f32>::with_capacity((res[0] * res[1] * res[2]) as usize);

let mut k = 0;
let mut z: f32 = dims[2][0];
while k < res[2] {
    let mut j = 0;
    let mut y = dims[1][0] - dims[1][2];
    while j < res[1] {
        let mut i = 0;
        let mut x = dims[0][0] - dims[0][2];
        while i < res[0] {
            volume.push(f(x, y, z));
            i += 1;
            x += dims[0][2];
        }
        j += 1;
        y += dims[1][2];
    }
    k += 1;
    z += dims[2][2];
}
        SurfaceNetData::new(volume, res)
    }
}
