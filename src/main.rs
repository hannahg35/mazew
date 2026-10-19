/*
By: <Hannah Ursula Greig>
Date: 2026-10-01
Program Details: <A maze game with moving walls and multiscreens to review my GUI skills. A custom player module was made to learn how to make custom modules.>
*/
mod custom;
mod ui;
mod utils;
use crate::custom::player::Player;
use crate::ui::grid::draw_grid;
use crate::ui::image_button::ImageButton;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::collision::check_collision;
use crate::utils::preload_image::GifLoadingScreenInfo;
use crate::utils::preload_image::LoadingScreenOptions;
use crate::utils::preload_image::TextureManager;
use crate::utils::scale::use_virtual_resolution;

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
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tm = TextureManager::new();
    let load_background_color = Color::from_hex(0x9ecfe6);
    let load_color = Color::from_hex(0x569fc2);
    let font = load_ttf_font("assets/roca.ttf").await.unwrap();
    let loading_options = LoadingScreenOptions {
        title: Some("Grab it Rabbit".to_string()),
        background_color: load_background_color,
        bar_fill_color: load_color,
        ..Default::default()
    };
    tm.preload_with_loading_screen(
        &[
            "assets/border.png",
            "assets/maze.png",
            "assets/carrot.png",
            "assets/horizontalwall.png",
            "assets/verticalwall.png",
            "assets/exit.png",
            "assets/exitpress.png",
            "assets/play.png",
            "assets/playpress.png",
            "assets/rabbit.png",
            "assets/again.png",

            "assets/againpress.png",
        ],
        None,
        None,
    )
    .await;

    let btn_exit = ImageButton::from_preload(
        2275.0,
        50.0,
        100.0,
        100.0,
        tm.get_preload("assets/exit.png").unwrap(),
        tm.get_preload("assets/exitpress.png").unwrap(),
    );

    let background_color = Color::from_hex(0x3d2a20);

    let mut img_border = StillImage::new("", 2350.0, 1150.0, 0.0, 0.0, true, 1.0).await;
    img_border.set_preload(tm.get_preload("assets/border.png").unwrap());
    let mut img_maze = StillImage::new("", 2350.0, 1150.0, 0.0, 0.0, true, 1.0).await;
    img_maze.set_preload(tm.get_preload("assets/maze.png").unwrap());


    let mut img_carrot1 = StillImage::new("", 100.0, 85.0, 475.0, 85.0, true, 1.0).await;
    img_carrot1.set_preload(tm.get_preload("assets/carrot.png").unwrap());
    let mut img_carrot2 = StillImage::new("", 100.0, 85.0, 325.0, 455.0, true, 1.0).await;
    img_carrot2.set_preload(tm.get_preload("assets/carrot.png").unwrap());
    let mut img_carrot3 = StillImage::new("", 100.0, 85.0, 85.0, 85.0, true, 1.0).await;
    img_carrot3.set_preload(tm.get_preload("assets/carrot.png").unwrap());
    let mut img_carrot4 = StillImage::new("", 100.0, 85.0, 1660.0, 600.0, true, 1.0).await;
    img_carrot4.set_preload(tm.get_preload("assets/carrot.png").unwrap());
    let mut img_carrot5 = StillImage::new("", 100.0, 85.0, 95.0, 775.0, true, 1.0).await;
    img_carrot5.set_preload(tm.get_preload("assets/carrot.png").unwrap());
    let mut img_carrot6 = StillImage::new("", 100.0, 85.0, 1175.0, 455.0, true, 1.0).await;
    img_carrot6.set_preload(tm.get_preload("assets/carrot.png").unwrap());
    let mut img_carrot7 = StillImage::new("", 100.0, 85.0, 1075.0, 85.0, true, 1.0).await;
    img_carrot7.set_preload(tm.get_preload("assets/carrot.png").unwrap());
    let mut img_carrot8 = StillImage::new("", 100.0, 85.0, 1375.0, 925.0, true, 1.0).await;
    img_carrot8.set_preload(tm.get_preload("assets/carrot.png").unwrap());
    let mut img_carrot9 = StillImage::new("", 100.0, 85.0, 1900.0, 250.0, true, 1.0).await;
    img_carrot9.set_preload(tm.get_preload("assets/carrot.png").unwrap());
    let mut img_carrot10 = StillImage::new("", 100.0, 85.0, 2175.0, 550.0, true, 1.0).await;
    img_carrot10.set_preload(tm.get_preload("assets/carrot.png").unwrap());

    let mut img_wall1 = StillImage::new("", 150.0, 75.0, 650.0, 185.0, true, 1.0).await;
    img_wall1.set_preload(tm.get_preload("assets/horizontalwall.png").unwrap());
    let mut img_wall2 = StillImage::new("", 75.0, 150.0, 750.0, 400.0, true, 1.0).await;
    img_wall2.set_preload(tm.get_preload("assets/verticalwall.png").unwrap());
    let mut img_wall3 = StillImage::new("", 75.0, 150.0, 1550.0, 400.0, true, 1.0).await;
    img_wall3.set_preload(tm.get_preload("assets/verticalwall.png").unwrap());

    let mut player = Player::new(2150.0, 1000.0, 80.0, 80.0, "assets/rabbit.png", 300.0).await;
    let mut carrots = 0;
    let mut lbl_carrots = Label::new("Carrots: 0", 25.0, 75.0, 60);
    lbl_carrots.with_font(font);

    let mut lbl_out = Label::new("That isn't enough food for everyone,\nwe can't leave yet!", 600.0, 1000.0, 60);
    lbl_out.with_font((font).clone());
    lbl_out.set_visible(false);

    let mut wall_hor = false;
    let mut wall_ver = false;

    loop {
        use_virtual_resolution(2350.0, 1150.0);
        clear_background(background_color);

        player.keypress();
        player.move_player();
        player.move_player();

        if player.get_image().get_y() >= 1100.0 && carrots != 10 {
            lbl_out.set_visible(true);
            player.move_speed = 0.0;
        } else if player.get_image().get_y() >= 1100.0 && carrots == 10 {
            lbl_out.set_visible(false);
            player.move_speed = 300.0;
        }

        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &img_maze, 1) {
                player.move_back_x();
            }
        }
        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &img_maze, 1) {
                player.move_back_y();
            }
        }

        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &img_wall1, 1) {
                player.move_back_x();
            }
        }
        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &img_wall1, 1) {
                player.move_back_y();
            }
        }

        if check_collision(player.get_image(), &img_carrot1, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot1.clear();
        }

        if check_collision(player.get_image(), &img_carrot2, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot2.clear();
        }

        if check_collision(player.get_image(), &img_carrot3, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot3.clear();
        }

        if check_collision(player.get_image(), &img_carrot4, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot4.clear();
        }

        if check_collision(player.get_image(), &img_carrot5, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot5.clear();
        }

        if check_collision(player.get_image(), &img_carrot6, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot6.clear();
        }

        if check_collision(player.get_image(), &img_carrot7, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot7.clear();
        }

        if check_collision(player.get_image(), &img_carrot8, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot8.clear();
        }

        if check_collision(player.get_image(), &img_carrot9, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot9.clear();
        }

        if check_collision(player.get_image(), &img_carrot10, 1) {
            carrots += 1;
            lbl_carrots.set_text(&format!("Carrots: {}", carrots));
            img_carrot10.clear();
        }

        let wall1_x = img_wall1.get_x();
        if wall1_x <= 400.0 {
            wall_hor = true;
        }
        if wall_hor == true {
            img_wall1.set_x(wall1_x + 1.0);
        }
        if wall_hor == false {
            img_wall1.set_x(wall1_x - 1.0);
        }
        if img_wall1.get_x() >= 1700.0 {
            wall_hor = false;
        }

        let wall2_y = img_wall2.get_y();
        if wall2_y <= 400.0 {
            wall_ver = true;
        }
        if wall_ver == true {
            img_wall2.set_y(wall2_y + 1.0);
        }
        if wall_ver == false {
            img_wall2.set_y(wall2_y - 1.0);
        }
        if img_wall2.get_y() >= 1100.0 {
            wall_ver = false;
        }

        let wall3_y = img_wall3.get_y();
        if wall3_y <= 400.0 {
            wall_ver = true;
        }
        if wall_ver == true {
            img_wall3.set_y(wall3_y + 1.0);
        }
        if wall_ver == false {
            img_wall3.set_y(wall3_y - 1.0);
        }
        if img_wall3.get_y() >= 850.0 {
            wall_ver = false;
        }

        player.draw();
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
        lbl_carrots.draw();
        img_wall1.draw();
        img_wall2.draw();
        img_wall3.draw();
        if btn_exit.click() {
            break;
        }
        //draw_grid(50.0, BLUE);
        next_frame().await;
    }
}
