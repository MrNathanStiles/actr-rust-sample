pub mod surface_net;
pub mod surface_net_generator;
pub mod surface_net_data;
// surface-net named files in the project are covered by these notices
// The MIT License (MIT)
//
// Copyright (c) 2012-2013 Mikola Lysenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
/**
 * SurfaceNets in JavaScript ported to Assembly Script ported to rust
 *
 * Written by Mikola Lysenko (C) 2012
 *
 * Ported by Nathan Stiles
 * 
 * MIT License
 *
 * Based on: S.F. Gibson, "Constrained Elastic Surface Nets". (1998) MERL Tech Report.
 */


 type PointDataGen = fn(x: f32, y: f32, z: f32) -> f32;

pub struct Precompute {
    cube_edges: Vec<usize>,
    edge_table: Vec<usize>,
}
 
pub fn precompute_edge_table() -> Precompute {

    let mut cube_edges: Vec<usize> = vec![0; 24];
    let mut edge_table: Vec<usize> = vec![0; 256];
    
     // Initialize the cube_edges table
     // This is just the vertex number of each cube
     let mut k: usize = 0;
     let mut i: usize = 0;
     let mut j: usize = 0;
     while i < 8 {
        j = 1;
        loop {
            let p = i ^ j;
            if i <= p {
                cube_edges[k] = i;
                k += 1;
                cube_edges[k] = p;
                k += 1;
            }
            
            j <<= 1;
            if j > 4 {
                break;
            }
        }
        i += 1;
     }
 
     // Initialize the intersection table.
     // This is a 2^(cube configuration) ->  2^(edge configuration) map
     // There is one entry for each possible cube configuration, and the output is a 12-bit vector enumerating all edges crossing the 0-level.
     for i in 0..256 {
         let mut em = 0;
         let mut j = 0;
         while j < 24 {
             let a = !!(i & (1 << cube_edges[j]));
             let b = !!(i & (1 << cube_edges[j + 1]));
             em |= if a != b { (1 << (j >> 1)) } else {0};
             j += 2;
         }
         edge_table[i] = em;
         
     }
     Precompute {
        cube_edges,
        edge_table
     }
 }
 
 
 
 /*
     todo this is scale by distance note that need to be implemented somewhere
     In that case you should be able to just scale it by (1000/distance I want it to look like).
     e.g. if you want it to look like it's 50,000 units away, put it 1000 units away and scale by 1/50.
 */
 