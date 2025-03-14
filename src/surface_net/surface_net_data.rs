use std::fmt::format;

use crate::{actr::_actr_log_length, component::vector3::Vector3};

use super::{surface_net::SurfaceNet, Precompute};


pub struct SurfaceNetData {
    pub data: Vec<f32>,
    pub dims: Vec<usize>,
}

pub fn log(message: String) {
    unsafe {
        _actr_log_length(message.as_ptr(), message.len());
    }
}

impl SurfaceNetData {
    pub fn new(data: Vec<f32>, dims: Vec<usize>) -> SurfaceNetData {
        SurfaceNetData {
            data,
            dims,
        }
    }

    pub fn generate_net(&self, precompute: Precompute) -> SurfaceNet {
        let dims = &self.dims;
        let data = &self.data;

        let mut n = 0;
        let mut x: Vec<usize> = vec![0,0,0];
        
        let mut grid: Vec<f32> = vec![0.0; 8];
        
        // check ^= over usize and i32
        let mut buf_no: usize = 1;
        let mut vertices: Vec<f32> = Vec::new();
        let mut faces: Vec<usize> = Vec::new();

        let mut root: Vec<isize> = vec![1, (dims[0] + 1) as isize, ((dims[0] + 1) * (dims[1] + 1)) as isize];
        log(format!("root {} {} {}", root[0], root[1], root[2]));
        // Resize buffer if necessary 
        let mut buffer: Vec<usize> = if root[2] * 2 > 4096 {
            vec![0; (root[2] * 2) as usize]
        } else {
            vec![0; 4096]
        };
        let mut push_count: usize = 0;
        let large: f32 = 999999999.0;

        let mut minx = large;
        let mut miny = large;
        let mut minz = large;
        
        let mut maxx = -large;
        let mut maxy = -large;
        let mut maxz = -large;
        
        // March over the voxel grid
        while x[2] < dims[2] - 1 {

            log(format!("generate net"));

            // m is the pointer into the buffer we are going to use.  
            // This is slightly obtuse because javascript does not have good support for packed data structures, so we must use typed arrays :(
            // The contents of the buffer will be the indices of the vertices on the previous x/y slice of the volume
            let mut moot: isize = (1 + (dims[0] + 1) * (1 + buf_no * (dims[1] + 1))) as isize;
            let mut i: usize = 0;
            let mut j: usize = 0;
            x[1] = 0;
            while x[1] < dims[1] - 1 {
                x[0] = 0;
                while x[0] < dims[0] - 1 {

                    //Read in 8 field values around this vertex and store them in an array
                    //Also calculate 8-bit mask, like in marching cubes, so we can speed up sign checks later
                    let mut mask: u8 = 0;
                    let mut g: u8 = 0;
                    let mut idx = n;
                    let mut k = 0;
                    while k < 2 {
                        j = 0;
                        while j < 2 {
                            i = 0;
                            while i < 2 {
                                let p = data[idx];
                                
                                grid[g as usize] = p;
                                mask |= if p < 0.0 { 1 << g } else { 0 };
                                i += 1;
                                g += 1;
                                idx += 1;
                            }
                            j += 1;
                            idx += (dims[0] - 2);
                        }
                        k += 1;
                        idx += (dims[0] * (dims[1] - 2));
                    }
                    //Check for early termination if cell does not intersect boundary
                    if mask == 0 || mask == 0xff {
                        x[0] += 1;
                        n += 1;
                        moot += 1;
                        continue;
                    }
                    
                    

                    //Sum up edge intersections
                    let edge_mask = precompute.edge_table[mask as usize];
                        //, v = [0.0, 0.0, 0.0]
                    let mut e_count = 0;

                    let mut v: Vec<f32> = vec![0.0, 0.0, 0.0];


                    //For every edge of the cube...
                    i = 0;
                    while i < 12 {

                        //Use edge mask to check if it is crossed
                        if (edge_mask & (1 << i)) == 0 {
                            i += 1;
                            continue;
                        }

                        //If it did, increment number of edge crossings
                        e_count += 1;

                        //Now find the point of intersection
                        let e0 = precompute.cube_edges[i << 1];       //Unpack vertices
                        let e1 = precompute.cube_edges[(i << 1) + 1];
                        let g0 = grid[e0 as usize];                 //Unpack grid values
                        let g1 = grid[e1 as usize];
                        let mut t = g0 - g1;                 //Compute point of intersection
                        if t.abs() > 1e-6 {
                            t = g0 / t;
                        } else {
                            i += 1;
                            continue;
                        }

                        //Interpolate vertices and add up intersections (this can be done without multiplying)
                        j = 0;
                        k = 1;
                        while j < 3 {
                            let a = e0 & k;
                            let b = e1 & k;
                            if a != b {
                                v[j] += if a != 0 { 1.0 - t } else { t };
                            } else {
                                v[j] += if a != 0 { 1.0 } else { 0.0 };
                            }
                            j += 1;
                            k <<= 1;
                        }
                        i += 1;
                    }

                    //Now we just average the edge intersections and add them to coordinate
                    let s: f32 = 1.0 / e_count as f32;
                    i = 0;
                    while i < 3 {
                        let foo  = x[i] as f32 + s * v[i]; 
                        v[i] = foo;
                        i += 1;
                    }

                    //Add vertex to buffer, store pointer to vertex index in buffer
                    // log(format!("b {} {}", moot, push_count));
                    buffer[moot as usize] = push_count;
                    push_count += 1;

                    minx = v[0].min(minx);
                    miny = v[1].min(miny);
                    minz = v[2].min(minz);
                    
                    maxx = v[0].max(maxx);
                    maxy = v[1].max(maxy);
                    maxz = v[2].max(maxz);
                    
                    vertices.push(v[0]);
                    vertices.push(v[1]);
                    vertices.push(v[2]);

                    //Now we need to add faces together, to do this we just loop over 3 basis components
                    i = 0;
                    while i < 3 {
                        //The first three entries of the edge_mask count the crossings along the edge
                        if (edge_mask & (1 << i)) == 0 {
                            i += 1;
                            continue;
                        }

                        // i = axes we are point along.  iu, iv = orthogonal axes
                        let iu = (i + 1) % 3;
                        let iv = (i + 2) % 3;

                        //If we are on a boundary, skip it
                        if x[iu] == 0 || x[iv] == 0 {
                            i += 1;
                            continue;
                        }

                        //Otherwise, look up adjacent edges in buffer
                        let du: isize = root[iu];
                        let dv: isize = root[iv];
                        //log(format!("root {} {} {} du {} dv {}", root[0], root[1], root[2], du, dv));
                        if moot < du || moot < dv || moot < dv + du {
                            log(format!("fail moot {moot} du {du} dv {dv}"));
                        }

                        //Remember to flip orientation depending on the sign of the corner.
                        //log(format!("mask {} {}", mask, mask & 1));
                        if (mask & 1) != 0 {
                            //log(format!("mask1 {}", mask));
                            //log(format!("i1 {} {} {} {} {} {}", moot, moot - du, moot - du - dv, moot, moot - du - dv, moot - dv));
                            faces.push(buffer[moot as usize]);                            
                            faces.push(buffer[(moot - du) as usize]);
                            faces.push(buffer[(moot - du - dv) as usize]);
                            faces.push(buffer[moot as usize]);
                            faces.push(buffer[(moot - du - dv) as usize]);
                            faces.push(buffer[(moot - dv) as usize]);

                        } else {
                            //log(format!("mask2 {}", mask));
                            faces.push(buffer[moot as usize]);
                            faces.push(buffer[(moot - dv) as usize]);
                            faces.push(buffer[(moot - du - dv) as usize]);
                            faces.push(buffer[moot as usize]);
                            faces.push(buffer[(moot - du - dv) as usize]);
                            faces.push(buffer[(moot - du) as usize]);
                        }
                        i += 1;
                    }
                    x[0] += 1;
                    n += 1;
                    moot += 1;
                }
                x[1] += 1;
                n += 1;
                moot += 2;
            }

            x[2] += 1;
            n += dims[0];
            buf_no ^= 1;
            root[2] = -root[2];
        }

        let size = Vector3::new((maxx - minx) as f64, (maxy - miny) as f64, (maxz - minz) as f64);
        let fx: f32 = minx + (size.x / 2.0) as f32;
        let fy: f32 = miny + (size.y / 2.0) as f32;
        let fz: f32 = minz + (size.z / 2.0) as f32;
        let mut i = 0;
        while i < vertices.len() {
            vertices[i] -= fx;
            vertices[i + 1] -= fy;
            vertices[i + 2] -= fz;

            // vertices[i] -= mx;
            // vertices[i + 1] -= my;
            // vertices[i + 2] -= mz;
            i += 3
        }
        // All done!  Return the result
        SurfaceNet::new(vertices, faces, size)
    }
}
