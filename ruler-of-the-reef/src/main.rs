use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let shark_textures = vec![
        load_texture("./src/assets/Shark_move_1_000.png")
            .await
            .unwrap(),
        load_texture("./src/assets/Shark_move_1_001.png")
            .await
            .unwrap(),
        load_texture("./src/assets/Shark_move_1_002.png")
            .await
            .unwrap(),
        load_texture("./src/assets/Shark_move_1_003.png")
            .await
            .unwrap(),
        load_texture("./src/assets/Shark_move_1_004.png")
            .await
            .unwrap(),
        load_texture("./src/assets/Shark_move_1_005.png")
            .await
            .unwrap(),
        load_texture("./src/assets/Shark_move_1_006.png")
            .await
            .unwrap(),
        load_texture("./src/assets/Shark_move_1_007.png")
            .await
            .unwrap(),
        load_texture("./src/assets/Shark_move_1_008.png")
            .await
            .unwrap(),
        load_texture("./src/assets/Shark_move_1_009.png")
            .await
            .unwrap(),
    ];

    let texture = load_texture("./src/1_game_background.png").await.unwrap();
    //let sprite_sheet = load_texture("./src/spritesheet.png").await.unwrap();
    // let tiled_map_json = load_string("./src/omg2.json").await.unwrap();
    //let tileset = load_texture("./src/omg2.png").await.unwrap();
    //tileset.set_filter(FilterMode::Nearest);

    // let crabTexture = load_texture("./src/Crab_attack_1_000.png").await.unwrap();
    // let sharkTexture = load_texture("./src/Shark_attack_1_000.png").await.unwrap();
    // let tiled_map = tiled::load_map(&tiled_map_json, &[("omg2.png", tileset)], &[]).unwrap();
    //let mut updImage = true;

    let mut frame_count = 0;
    let mut animation_frame_count = 0;
    let mut shark_x = 0.0;
    let mut shark_flip_x = false;
    let mut shark_y = 0.0;

    loop {
        let w = screen_width();
        let h = screen_height();

        clear_background(WHITE);
        draw_texture(&texture, 0., 0., WHITE);

        let current_shark_texture = &shark_textures[animation_frame_count];

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            shark_x += 5.;
            shark_flip_x = true;
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            shark_x -= 5.;
            shark_flip_x = false;
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            shark_y += 2.;
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            shark_y -= 2.;
        }

        let shark_texture_params = DrawTextureParams {
            flip_x: shark_flip_x,
            dest_size: Some(vec2(1075. / 3., 529. / 3.)),
            ..Default::default()
        };

        draw_texture_ex(
            current_shark_texture,
            shark_x,
            shark_y,
            WHITE,
            shark_texture_params,
        );

        frame_count += 1;
        frame_count = frame_count % 1000;

        if frame_count % 10 == 0 {
            animation_frame_count += 1;
            animation_frame_count = animation_frame_count % 10;
        }

        next_frame().await
    }
}
