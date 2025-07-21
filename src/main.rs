use std::f32::consts::PI;

use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

const Z_COUNT: usize = 100;
const X_COUNT: usize = 100;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut grid: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); X_COUNT * Z_COUNT];

    let perlin = Perlin::new(0);
    let scale = 2.5;

    for z in 0..Z_COUNT {
        for x in 0..X_COUNT {
            grid[z * X_COUNT + x].x = x as f32;

            grid[z * X_COUNT + x].y +=
                perlin.get([x as f64 * scale, 0.0, z as f64 * scale]) as f32 * 2.5;

            grid[z * X_COUNT + x].z = z as f32;
        }
    }

    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up = right.cross(front).normalize();

    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = false;
    set_cursor_grab(grabbed);
    show_mouse(!grabbed);

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_down(KeyCode::W) {
            position += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            position -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;

        last_mouse_position = mouse_position;

        if grabbed {
            yaw += mouse_delta.x * delta * LOOK_SPEED;
            pitch += mouse_delta.y * delta * -LOOK_SPEED;

            pitch = if pitch > 1.5 { 1.5 } else { pitch };
            pitch = if pitch < -1.5 { -1.5 } else { pitch };

            front = vec3(
                yaw.cos() * pitch.cos(),
                pitch.sin(),
                yaw.sin() * pitch.cos(),
            )
            .normalize();

            right = front.cross(world_up).normalize();
            up = right.cross(front).normalize();

            x += if switch { 0.04 } else { -0.04 };
            if x >= bounds || x <= -bounds {
                switch = !switch;
            }
        }

        clear_background(DARKGRAY);

        // Going 3d!

        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            fovy: PI / 180.0 * 100.0,
            ..Default::default()
        });

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
            }
        }

        // Back to screen space, render some text
        set_default_camera();

        next_frame().await
    }
}
