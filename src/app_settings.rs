use macroquad::prelude::*;

pub fn conf() -> Conf {
    Conf {
        window_title: String::from("Terrain Generator"),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}
