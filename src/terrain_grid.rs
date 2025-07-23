use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
pub struct TerrainGrid {
    pub x_count: usize,
    pub z_count: usize,
    pub grid: Vec<Vec3>,
}

impl TerrainGrid {
    pub fn new(x_count: usize, z_count: usize) -> Self {
        let mut grid: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); x_count * z_count];

        let perlin = Perlin::new(10);
        let scale = 0.2;

        for z in 0..z_count {
            for x in 0..x_count {
                grid[z * x_count + x].x = x as f32;
                grid[z * x_count + x].y +=
                    perlin.get([x as f64 * scale, 0.0, z as f64 * scale]) as f32;
                grid[z * x_count + x].z = z as f32;
            }
        }

        TerrainGrid {
            x_count: x_count,
            z_count: z_count,
            grid: grid,
        }
    }
}
