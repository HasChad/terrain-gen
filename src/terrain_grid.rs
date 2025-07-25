use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

const CHUNK_SIZE: usize = 28;

pub struct Terrain {
    pub meshes: Vec<Mesh>,
}

impl Terrain {
    pub fn new(x_mesh_count: usize, z_mesh_count: usize, brick_texture: Texture2D) -> Self {
        let x_vert_count = x_mesh_count + 1;
        let z_vert_count = z_mesh_count + 1;

        let grid_scale: f32 = 1.0;
        let mut scale: f64 = 0.02 / grid_scale as f64;
        let mut height: f32 = 30.0 * grid_scale;

        let mut grid: Vec<Vec3> = vec![Vec3::ZERO; x_vert_count * z_vert_count];

        let perlin = Perlin::new(1);

        for z in 0..z_vert_count {
            for x in 0..x_vert_count {
                let x_test = x as f32 * grid_scale;
                let z_test = z as f32 * grid_scale;

                grid[z * x_vert_count + x].x = x_test as f32;
                grid[z * x_vert_count + x].y +=
                    perlin.get([x_test as f64 * scale, 0.0, z_test as f64 * scale]) as f32 * height;
                grid[z * x_vert_count + x].z = z_test as f32;
            }
        }

        scale = 0.2;
        height = 1.0;

        for cell in grid.iter_mut() {
            cell.y +=
                perlin.get([cell.x as f64 * scale, 0.0, cell.z as f64 * scale]) as f32 * height;
        }

        let mut meshes = Vec::new();

        let mesh_color = YELLOW;

        for cz in (0..z_mesh_count).step_by(CHUNK_SIZE) {
            for cx in (0..x_mesh_count).step_by(CHUNK_SIZE) {
                let chunk_x_count = CHUNK_SIZE.min(x_mesh_count - cx);
                let chunk_z_count = CHUNK_SIZE.min(z_mesh_count - cz);

                let quad_count = chunk_x_count * chunk_z_count;
                let mut vertices = Vec::with_capacity(quad_count * 4);
                let mut indices = Vec::with_capacity(quad_count * 6);
                let mut index_offset = 0;

                for z in 0..chunk_z_count {
                    for x in 0..chunk_x_count {
                        let gx = cx + x;
                        let gz = cz + z;
                        let i = gz * x_vert_count + gx;

                        let v0 = grid[i];
                        let v1 = grid[i + 1];
                        let v2 = grid[i + x_vert_count];
                        let v3 = grid[i + 1 + x_vert_count];

                        let normal3 = (v1 - v0).cross(v2 - v0).normalize();
                        let normal = vec4(normal3.x, -normal3.y, normal3.z, 0.0);

                        vertices.extend_from_slice(&[
                            Vertex {
                                position: v0,
                                uv: vec2(1., 0.),
                                color: mesh_color.into(),
                                normal,
                            },
                            Vertex {
                                position: v1,
                                uv: vec2(0., 0.),
                                color: mesh_color.into(),
                                normal,
                            },
                            Vertex {
                                position: v2,
                                uv: vec2(1., 1.),
                                color: mesh_color.into(),
                                normal,
                            },
                            Vertex {
                                position: v3,
                                uv: vec2(0., 1.),
                                color: mesh_color.into(),
                                normal,
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

                if !vertices.is_empty() {
                    meshes.push(Mesh {
                        vertices,
                        indices,
                        texture: Some(brick_texture.clone()),
                    });
                }
            }
        }

        Terrain { meshes }
    }

    pub fn draw(self: &Self) {
        for mesh in &self.meshes {
            draw_mesh(&mesh);
        }
    }
}
