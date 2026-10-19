use crate::ui::still_image::StillImage;
use ::macroquad::input::KeyCode;
use ::macroquad::prelude::*;

pub struct Player {
    image: StillImage,
    pub move_speed: f32,
    pub move_dir: Vec2,
    pub old_position: Vec2,
}
impl Player {
    pub async fn new(x: f32, y: f32, width: f32, height: f32, image_name: &str, move_speed: f32) -> Player {
        let img_image = StillImage::new(image_name , width, height, x, y, true, 1.0).await;
        Player {
            image: img_image,
            move_speed,
            move_dir: vec2(0.0, 0.0),
            old_position: vec2(0.0, 0.0),
        }
    }
    pub fn get_image(&self) -> &StillImage {
        &self.image
    }

    pub fn keypress(&mut self) {
        self.move_dir = vec2(0.0, 0.0);
        self.old_position = vec2(self.image.get_x(), self.image.get_y());

        if is_key_down(KeyCode::D) {
            self.move_dir.x += 2.0;
        }
        if is_key_down(KeyCode::A) {
            self.move_dir.x -= 2.0;
        }
        if is_key_down(KeyCode::S) {
            self.move_dir.y += 2.0;
        }
        if is_key_down(KeyCode::W) {
            self.move_dir.y -= 2.0;
        }

        if self.move_dir.length() > 0.0 {
            self.move_dir = self.move_dir.normalize();
        }
    }

    pub fn draw(&self) {
        self.image.draw();
    }
    pub fn move_player(&mut self) {
        self.image.set_x(self.image.get_x() + self.move_dir.x);
        self.image.set_y(self.image.get_y() + self.move_dir.y);
    }
    pub fn move_back_x(&mut self) {
        self.image.set_x(self.old_position.x);
    }
    pub fn move_back_y(&mut self) {
        self.image.set_y(self.old_position.y);
    }
}
