use macroquad::prelude::*;

const LOOK_SPEED: f32 = 50.0;
const MOVE_SPEED: f32 = 0.1;

pub struct Player {
    pub yaw: f32,
    pub pitch: f32,
    pub front: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub position: Vec3,
}

impl Player {
    pub fn new() -> Self {
        let world_up = vec3(0.0, 1.0, 0.0);
        let yaw: f32 = 1.18;
        let pitch: f32 = 0.0;
        let front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        let right = front.cross(world_up).normalize();
        let up = right.cross(front).normalize();

        Player {
            yaw: yaw,
            pitch: pitch,
            front: front,
            right: right,
            up: up,
            position: vec3(0.0, 1.0, 0.0),
        }
    }

    pub fn move_head(self: &mut Self) {
        let mouse_delta = mouse_delta_position();
        let delta = get_frame_time();
        let world_up = vec3(0.0, 1.0, 0.0);

        self.yaw += mouse_delta.x * delta * -LOOK_SPEED;
        self.pitch += mouse_delta.y * delta * LOOK_SPEED;

        self.pitch = if self.pitch > 1.5 { 1.5 } else { self.pitch };
        self.pitch = if self.pitch < -1.5 { -1.5 } else { self.pitch };

        self.front = vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();

        self.right = self.front.cross(world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }

    pub fn move_player(self: &mut Self) {
        let player_front = Vec3::new(self.front.x, 0.0, self.front.z).normalize();

        if is_key_down(KeyCode::W) {
            self.position += player_front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            self.position -= player_front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            self.position -= self.right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            self.position += self.right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Space) {
            self.position.y += MOVE_SPEED;
        }
        if is_key_down(KeyCode::LeftControl) {
            self.position.y -= MOVE_SPEED;
        }
    }
}
