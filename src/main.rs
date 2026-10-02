/*
By: <Your Name Here>
Date: 2026-10-01
Program Details: <Program Description Here>
*/

mod ui;
mod utils;
use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::collision::check_collision;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "maze2".to_string(),
        window_width: 2350,
        window_height: 1150,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut img_player = StillImage::new("assets/rabbit.png", 80.0, 80.0, 2150.0, 1025.0, true, 1.0).await;
    let background_color = Color::from_hex(0x3d2a20);
    let img_border = StillImage::new("assets/border.png", 2350.0, 1150.0, 0.0, 0.0, true, 1.0).await;
    let img_maze = StillImage::new("assets/maze.png", 2350.0, 1150.0, 0.0, 0.0, true, 1.0).await;
    let mut img_carrot1 = StillImage::new("assets/carrot.png", 100.0, 85.0, 475.0, 85.0, true, 1.0).await;
    let mut img_carrot2 = StillImage::new("assets/carrot.png", 100.0, 85.0, 325.0, 455.0, true, 1.0).await;
    let mut img_carrot3 = StillImage::new("assets/carrot.png", 100.0, 85.0, 85.0, 85.0, true, 1.0).await;
    let mut img_carrot4 = StillImage::new("assets/carrot.png", 100.0, 85.0, 1660.0, 600.0, true, 1.0).await;
    let mut img_carrot5 = StillImage::new("assets/carrot.png", 100.0, 85.0, 95.0, 775.0, true, 1.0).await;
    let mut img_carrot6 = StillImage::new("assets/carrot.png", 100.0, 85.0, 1175.0, 455.0, true, 1.0).await;
    let mut img_carrot7 = StillImage::new("assets/carrot.png", 100.0, 85.0, 1075.0, 85.0, true, 1.0).await;
    let mut img_carrot8 = StillImage::new("assets/carrot.png", 100.0, 85.0, 1375.0, 925.0, true, 1.0).await;
    let mut img_carrot9 = StillImage::new("assets/carrot.png", 100.0, 85.0, 1900.0, 250.0, true, 1.0).await;
    let mut img_carrot10 = StillImage::new("assets/carrot.png", 100.0, 85.0, 2175.0, 550.0, true, 1.0).await;
    let img_wall1 = StillImage::new("assets/horizontalwall.png", 150.0, 70.0, 650.0, 200.0, true, 1.0).await;

    const MOVE_SPEED: f32 = 350.0;

    let font = load_ttf_font("assets/roca.ttf").await.unwrap();
    let mut carrots = 0;
    let mut lbl_carrots = Label::new("Carrots: 0", 25.0, 50.0, 60);
    lbl_carrots.with_font(font);

    loop {
        clear_background(background_color);

        let mut move_dir = vec2(0.0, 0.0);

        if is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
        }
        if is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            move_dir.y += 1.0;
        }
        if is_key_down(KeyCode::W) {
            move_dir.y -= 1.0;
        }

        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        let movement = move_dir * MOVE_SPEED * get_frame_time();

        let old_pos = img_player.pos();

        if movement.x != 0.0 {
            img_player.set_x(img_player.get_x() + movement.x);
            if check_collision(&img_player, &img_maze, 1) {
                img_player.set_x(old_pos.x);
            }
        }

        if movement.y != 0.0 {
            img_player.set_y(img_player.get_y() + movement.y);
            if check_collision(&img_player, &img_maze, 1) {
                img_player.set_y(old_pos.y);
            }
        }

        if check_collision(&img_player, &img_carrot1, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot1.clear();
        }

        if check_collision(&img_player, &img_carrot2, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot2.clear();
        }

        if check_collision(&img_player, &img_carrot3, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot3.clear();
        }
        if check_collision(&img_player, &img_carrot4, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot4.clear();
        }
        if check_collision(&img_player, &img_carrot5, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot5.clear();
        }
        if check_collision(&img_player, &img_carrot6, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot6.clear();
        }
        if check_collision(&img_player, &img_carrot7, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot7.clear();
        }
        if check_collision(&img_player, &img_carrot8, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot8.clear();
        }
        if check_collision(&img_player, &img_carrot9, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot9.clear();
        }
        if check_collision(&img_player, &img_carrot10, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot10.clear();
        }

        img_maze.draw();
        img_border.draw();
        img_carrot1.draw();
        img_carrot2.draw();
        img_carrot3.draw();
        img_carrot4.draw();
        img_carrot5.draw();
        img_carrot6.draw();
        img_carrot7.draw();
        img_carrot8.draw();
        img_carrot9.draw();
        img_carrot10.draw();
        img_wall1.draw();
        img_player.draw();
        lbl_carrots.draw();

        draw_grid(50.0, BLUE);
        next_frame().await;
    }
}
