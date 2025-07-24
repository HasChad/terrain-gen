use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
pub struct TerrainGrid {
    pub x_count: usize,
    pub z_count: usize,
    pub grid: Vec<Vec3>,
    pub meshes: Vec<Mesh>,
}

impl TerrainGrid {
    pub fn new(x_count: usize, z_count: usize, texture: Texture2D) -> Self {
        let mut grid: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); x_count * z_count];

        let perlin = Perlin::new(0);
        let scale = 0.2;

        for z in 0..z_count {
            for x in 0..x_count {
                grid[z * x_count + x].x = x as f32;
                grid[z * x_count + x].y +=
                    perlin.get([x as f64 * scale, 0.0, z as f64 * scale]) as f32;
                grid[z * x_count + x].z = z as f32;
            }
        }

        let mut meshes = vec![];

        for (index, pos) in grid.iter().enumerate() {
            let x = (index % x_count) as f32;
            let z = (index / x_count) as f32;

            if x == 0.0 && z == 0.0 {
                draw_sphere(Vec3::new(pos.x, pos.y, pos.z), 0.2, None, RED);
            }

            if x == (x_count - 1) as f32 && z == (z_count - 1) as f32 {
                draw_sphere(Vec3::new(pos.x, pos.y, pos.z), 0.2, None, BLUE);
            }

            if z < (z_count - 1) as f32 {
                if x < (x_count - 1) as f32 {
                    // Define the two triangles of the quad
                    let mesh_color = DARKBROWN;

                    let normal3 = (grid[index + 1] - *pos)
                        .cross(grid[index + x_count] - *pos)
                        .normalize();

                    let normal = vec4(normal3.x, -normal3.y, normal3.z, 0.0);

                    let vertices = vec![
                        Vertex {
                            position: *pos,
                            uv: vec2(1., 0.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                        Vertex {
                            position: grid[index + 1],
                            uv: vec2(0., 0.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                        Vertex {
                            position: grid[index + x_count],
                            uv: vec2(1., 1.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                        Vertex {
                            position: grid[index + 1 + x_count],
                            uv: vec2(0., 1.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                    ];

                    let mesh = Mesh {
                        vertices: vertices,
                        // Indices for two triangles: (0, 1, 2) and (1, 3, 2)
                        indices: vec![0, 2, 1, 1, 2, 3],
                        texture: Some(texture.clone()),
                    };

                    meshes.push(mesh);
                }
            }
        }

        TerrainGrid {
            x_count: x_count,
            z_count: z_count,
            grid: grid,
            meshes: meshes,
        }
    }

    pub fn draw_mesh(self: &Self) {
        for mesh in &self.meshes {
            draw_mesh(&mesh);
        }
    }
}
