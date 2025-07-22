use std::f32::consts::PI;

use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
use rayon::prelude::*;

mod app_settings;
mod player;

use app_settings::*;
use player::*;

const Z_COUNT: usize = 10;
const X_COUNT: usize = 10;

#[macroquad::main(conf)]
async fn main() {
    let brick_texture = load_texture("rock_brick.png").await.unwrap();
    brick_texture.set_filter(FilterMode::Nearest);

    let mut grid: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); X_COUNT * Z_COUNT];

    let perlin = Perlin::new(0);
    let scale = 0.5;

    for z in 0..Z_COUNT {
        for x in 0..X_COUNT {
            grid[z * X_COUNT + x].x = x as f32;
            grid[z * X_COUNT + x].y += perlin.get([x as f64 * scale, 0.0, z as f64 * scale]) as f32;
            grid[z * X_COUNT + x].z = z as f32;
        }
    }

    let mut player = Player::new();

    let mut grabbed = false;
    set_cursor_grab(grabbed);
    show_mouse(!grabbed);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if get_keys_down().len() > 0 {
            player.move_player();
        }

        if grabbed {
            player.move_head();
        }

        clear_background(DARKGRAY);

        // Going 3d!
        set_camera(&Camera3D {
            position: player.position,
            up: player.up,
            target: player.position + player.front,
            fovy: PI / 180.0 * 100.0,
            ..Default::default()
        });

        // grid.par_iter().enumerate().for_each(|(index, pos)| {
        for (index, pos) in grid.iter().enumerate() {
            let x = (index % X_COUNT) as f32;
            let z = (index / X_COUNT) as f32;

            if x == 0.0 && z == 0.0 {
                draw_sphere(Vec3::new(pos.x, pos.y, pos.z), 0.2, None, RED);
            }

            if x == (X_COUNT - 1) as f32 && z == (Z_COUNT - 1) as f32 {
                draw_sphere(Vec3::new(pos.x, pos.y, pos.z), 0.2, None, BLUE);
            }

            if x < (X_COUNT - 1) as f32 {
                draw_line_3d(
                    vec3(pos.x, pos.y, pos.z),
                    vec3(grid[index + 1].x, grid[index + 1].y, grid[index + 1].z),
                    YELLOW,
                );
            }

            if z < (Z_COUNT - 1) as f32 {
                draw_line_3d(
                    vec3(pos.x, pos.y, pos.z),
                    vec3(
                        grid[index + X_COUNT].x,
                        grid[index + X_COUNT].y,
                        grid[index + X_COUNT].z,
                    ),
                    YELLOW,
                );

                if x < (X_COUNT - 1) as f32 {
                    // Define the two triangles of the quad
                    let mesh_color = DARKBROWN;

                    let normal3 = (grid[index + 1] - *pos)
                        .cross(grid[index + X_COUNT] - *pos)
                        .normalize();

                    let normal = vec4(normal3.x, normal3.y, normal3.z, 0.0);

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
                            position: grid[index + X_COUNT],
                            uv: vec2(1., 1.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                        Vertex {
                            position: grid[index + 1 + X_COUNT],
                            uv: vec2(0., 1.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                    ];

                    let mesh = Mesh {
                        vertices: vertices,
                        // Indices for two triangles: (0, 1, 2) and (1, 3, 2)
                        indices: vec![0, 1, 2, 1, 3, 2],
                        texture: Some(brick_texture.clone()),
                    };

                    draw_mesh(&mesh);
                }
            }
        }

        // Back to screen space, render some text
        set_default_camera();

        draw_text(
            format!("fps = {}", get_fps()).as_str(),
            10.0,
            20.0,
            32.0,
            WHITE,
        );

        next_frame().await
    }
}
