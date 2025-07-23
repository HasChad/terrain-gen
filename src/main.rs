use std::f32::consts::PI;

use macroquad::prelude::*;
// use rayon::prelude::*;

mod app_settings;
mod player;
mod terrain_grid;

use app_settings::*;
use player::*;
use terrain_grid::*;

#[macroquad::main(conf)]
async fn main() {
    set_default_filter_mode(FilterMode::Nearest);
    let brick_texture = load_texture("rock_brick.png").await.unwrap();

    let terrain_grid = TerrainGrid::new(10, 10);

    let mut player = Player::new();

    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    let material = load_material(
        ShaderSource::Glsl {
            vertex: &load_string("vert.glsl").await.unwrap(),
            fragment: &load_string("frag.glsl").await.unwrap(),
        },
        MaterialParams {
            pipeline_params,
            ..Default::default()
        },
    )
    .unwrap();

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
        for (index, pos) in terrain_grid.grid.iter().enumerate() {
            let x = (index % terrain_grid.x_count) as f32;
            let z = (index / terrain_grid.x_count) as f32;

            if x == 0.0 && z == 0.0 {
                draw_sphere(Vec3::new(pos.x, pos.y, pos.z), 0.2, None, RED);
            }

            if x == (terrain_grid.x_count - 1) as f32 && z == (terrain_grid.z_count - 1) as f32 {
                draw_sphere(Vec3::new(pos.x, pos.y, pos.z), 0.2, None, BLUE);
            }

            if x < (terrain_grid.x_count - 1) as f32 {
                draw_line_3d(
                    vec3(pos.x, pos.y, pos.z),
                    vec3(
                        terrain_grid.grid[index + 1].x,
                        terrain_grid.grid[index + 1].y,
                        terrain_grid.grid[index + 1].z,
                    ),
                    YELLOW,
                );
            }

            if z < (terrain_grid.z_count - 1) as f32 {
                draw_line_3d(
                    vec3(pos.x, pos.y, pos.z),
                    vec3(
                        terrain_grid.grid[index + terrain_grid.x_count].x,
                        terrain_grid.grid[index + terrain_grid.x_count].y,
                        terrain_grid.grid[index + terrain_grid.x_count].z,
                    ),
                    YELLOW,
                );

                if x < (terrain_grid.x_count - 1) as f32 {
                    // Define the two triangles of the quad
                    let mesh_color = DARKBROWN;

                    let normal3 = (terrain_grid.grid[index + 1] - *pos)
                        .cross(terrain_grid.grid[index + terrain_grid.x_count] - *pos)
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
                            position: terrain_grid.grid[index + 1],
                            uv: vec2(0., 0.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                        Vertex {
                            position: terrain_grid.grid[index + terrain_grid.x_count],
                            uv: vec2(1., 1.),
                            color: mesh_color.into(),
                            normal: normal,
                        },
                        Vertex {
                            position: terrain_grid.grid[index + 1 + terrain_grid.x_count],
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

                    gl_use_material(&material);
                    draw_mesh(&mesh);
                    gl_use_default_material();
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
