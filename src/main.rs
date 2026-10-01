/*
By: <Your Name Here>
Date: 2026-10-01
Program Details: <Program Description Here>
*/

mod ui;
mod utils;
use crate::ui::grid::draw_grid;
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
    let mut player = StillImage::new("assets/rabbit.png", 100.0, 100.0, 2150.0, 1025.0, true, 1.0).await;
    let background_color = Color::from_hex(0x3d2a20);
    let img_border = StillImage::new("assets/border.png", 2350.0, 1150.0, 0.0, 0.0, true, 1.0).await;
    let img_maze = StillImage::new("assets/maze.png", 2350.0, 1150.0, 0.0, 0.0, true, 1.0).await;
    let img_carrot1 = StillImage::new("assets/carrot.png", 100.0, 85.0, 475.0, 85.0, true, 1.0).await;
    let img_carrot2 = StillImage::new("assets/carrot.png", 100.0, 85.0, 325.0, 455.0, true, 1.0).await;
    let img_carrot3 = StillImage::new("assets/carrot.png", 100.0, 85.0, 85.0, 85.0, true, 1.0).await;
    let img_carrot4 = StillImage::new("assets/carrot.png", 100.0, 85.0, 1660.0, 600.0, true, 1.0).await;
    let img_carrot5 = StillImage::new("assets/carrot.png", 100.0, 85.0, 95.0, 775.0, true, 1.0).await;
    let img_carrot6 = StillImage::new("assets/carrot.png", 100.0, 85.0, 1175.0, 455.0, true, 1.0).await;
    let img_carrot7 = StillImage::new("assets/carrot.png", 100.0, 85.0, 1075.0, 85.0, true, 1.0).await;
    let img_carrot8 = StillImage::new("assets/carrot.png", 100.0, 85.0, 1375.0, 925.0, true, 1.0).await;
    let img_carrot9 = StillImage::new("assets/carrot.png", 100.0, 85.0, 1900.0, 250.0, true, 1.0).await;
    let img_carrot10 = StillImage::new("assets/carrot.png", 100.0, 85.0, 2175.0, 550.0, true, 1.0).await;
    let img_wall1 = StillImage::new("assets/horizontalwall.png", 150.0, 70.0, 650.0, 200.0, true, 1.0).await;

    loop {
        clear_background(background_color);
        let mut x = 0.0;
        let mut y = 0.0;

        if is_key_down(KeyCode::Left) {
            x -= 2.0;
        }
        if is_key_down(KeyCode::Right) {
            x += 2.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 2.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 2.0;
        }

        player.set_x(x + player.get_x());
        player.set_y(y + player.get_y());

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
        player.draw();
        draw_grid(50.0, BLUE);
        next_frame().await;
    }
}
