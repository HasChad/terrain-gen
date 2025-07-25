use macroquad::prelude::*;
use noise::{Constant, NoiseFn};

const CHUNK_SIZE: usize = 25;
const SCALE: f64 = 1.0;
const HEIGHT: f32 = 1.0;

pub struct TerrainGrid {
    // pub x_count: usize,
    // pub z_count: usize,
    // pub grid: Vec<Vec3>,
    pub meshes: Vec<Mesh>,
}

impl TerrainGrid {
    pub fn new(x_count: usize, z_count: usize, _brick_texture: Texture2D) -> Self {
        let x_count = x_count + 1;
        let z_count = z_count + 1;

        let mut grid: Vec<Vec3> = vec![Vec3::ZERO; x_count * z_count];

        // let perlin = Perlin::new(0.0);
        let perlin = Constant::new(0.0);

        for z in 0..z_count {
            for x in 0..x_count {
                grid[z * x_count + x].x = x as f32;
                grid[z * x_count + x].y +=
                    perlin.get([x as f64 * SCALE, 0.0, z as f64 * SCALE]) as f32 * HEIGHT;
                grid[z * x_count + x].z = z as f32;
            }
        }

        /*
        for cell in grid.iter_mut() {
            cell.y +=
                perlin.get([cell.x as f64 * SCALE, 0.0, cell.z as f64 * SCALE]) as f32 * HEIGHT;
        }
        */

        let quad_count = (x_count - 1) * (z_count - 1);

        let mut vertices: Vec<Vertex> = Vec::with_capacity(quad_count * 4);
        let mut indices: Vec<u16> = Vec::with_capacity(quad_count * 6);
        let mut index_offset = 0;

        let mesh_color = YELLOW;

        for cz in (0..z_count - 1).step_by(CHUNK_SIZE) {
            for cx in (0..x_count - 1).step_by(CHUNK_SIZE) {}
        }

        for (index, pos) in grid.iter().enumerate() {
            let x = (index % x_count) as f32;
            let z = (index / x_count) as f32;

            if z < (z_count - 1) as f32 {
                if x < (x_count - 1) as f32 {
                    let v0 = *pos;
                    let v1 = grid[index + 1];
                    let v2 = grid[index + x_count];
                    let v3 = grid[index + x_count + 1];

                    let normal3 = (v1 - v0).cross(v2 - v0).normalize();

                    let normal = vec4(normal3.x, -normal3.y, normal3.z, 0.0);

                    vertices.extend_from_slice(&[
                        Vertex {
                            position: v0,
                            uv: vec2(1., 0.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                        Vertex {
                            position: v1,
                            uv: vec2(0., 0.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                        Vertex {
                            position: v2,
                            uv: vec2(1., 1.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                        Vertex {
                            position: v3,
                            uv: vec2(0., 1.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                    ]);

                    indices.extend_from_slice(&[
                        index_offset,
                        index_offset + 1,
                        index_offset + 2,
                        index_offset + 1,
                        index_offset + 3,
                        index_offset + 2,
                    ]);

                    index_offset += 4;
                }
            }
        }

        let mesh = Mesh {
            vertices,
            indices,
            texture: None, // Some(brick_texture)
        };

        TerrainGrid {
            // x_count,
            // z_count,
            // grid,
            meshes: vec![mesh],
        }
    }

    pub fn draw(self: &Self) {
        for mesh in &self.meshes {
            draw_mesh(&mesh);
        }
    }
}
