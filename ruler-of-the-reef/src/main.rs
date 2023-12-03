use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};
use macroquad_tiled as tiled;

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
    let tiled_map_json = load_string("./src/omg2.json").await.unwrap();
    //let tileset = load_texture("./src/omg2.png").await.unwrap();
    //tileset.set_filter(FilterMode::Nearest);

    let crabTexture = load_texture("./src/Crab_attack_1_000.png").await.unwrap();
    let sharkTexture = load_texture("./src/Shark_attack_1_000.png").await.unwrap();
    // let tiled_map = tiled::load_map(&tiled_map_json, &[("omg2.png", tileset)], &[]).unwrap();
    //let mut updImage = true;

    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[
            ("Crab_attack_1_000.png", crabTexture),
            ("Shark_attack_1_000.png", sharkTexture),
        ],
        &[],
    )
    .unwrap();

    let mut x = 0.0;
    let mut y = 0.0;
    let mut dir = 1.;

    let mut frame_count = 0;
    let mut animation_frame_count = 0;
    let mut shark_x = 0.0;
    let mut shark_flip_x = false;

    loop {
        let w = screen_width();
        let h = screen_height();

        clear_background(WHITE);
        draw_texture(&texture, 0., 0., WHITE);

        let current_shark_texture = &shark_textures[animation_frame_count];
       
        if is_key_down(KeyCode::Right) {
            shark_x += 5.;
            shark_flip_x = true;
        }

        if is_key_down(KeyCode::Left) {
            shark_x -= 5.;
            shark_flip_x = false;
        }

        let shark_texture_params = DrawTextureParams {
            flip_x: shark_flip_x,
            ..Default::default()
        };

        draw_texture_ex(
            current_shark_texture,
            shark_x,
            0.,
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
