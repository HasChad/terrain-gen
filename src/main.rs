use std::f32::consts::PI;

use macroquad::prelude::*;

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

    let terrain_grid = TerrainGrid::new(25, 25, brick_texture);

    let mut player = Player::new();

    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    let material = load_material(
        ShaderSource::Glsl {
            vertex: &load_string("src/vert.glsl").await.unwrap(),
            fragment: &load_string("src/frag.glsl").await.unwrap(),
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

        clear_background(BLACK);

        // Going 3d!
        set_camera(&Camera3D {
            position: player.position,
            up: player.up,
            target: player.position + player.front,
            fovy: PI / 180.0 * 100.0,
            ..Default::default()
        });

        gl_use_material(&material);
        terrain_grid.draw();
        gl_use_default_material();

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
