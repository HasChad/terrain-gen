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

    let brick_texture = load_texture("assets/rock_brick.png").await.unwrap();

    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    let material = load_material(
        ShaderSource::Glsl {
            vertex: &load_string("shaders/vert.glsl").await.unwrap(),
            fragment: &load_string("shaders/frag.glsl").await.unwrap(),
        },
        MaterialParams {
            pipeline_params,
            ..Default::default()
        },
    )
    .unwrap();

    let terrain_grid = Terrain::new(1000, 1000, brick_texture);

    let mut player = Player::new();

    let mut grabbed = false;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if grabbed {
            player.move_head();

            if get_keys_down().len() > 0 {
                player.move_player();
            }
        }

        // MARK: Draw
        clear_background(BLACK);

        // 3D world
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

        // Back to screen space
        set_default_camera();

        draw_text(
            format!("fps = {}", get_fps()).as_str(),
            5.0,
            15.0,
            16.0,
            WHITE,
        );

        next_frame().await
    }
}
