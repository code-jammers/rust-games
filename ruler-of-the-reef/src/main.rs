use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};
use macroquad_tiled as tiled;

#[macroquad::main("BasicShapes")]
async fn main() {
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
        &[("Crab_attack_1_000.png", crabTexture), ("Shark_attack_1_000.png", sharkTexture)],
        &[],
    ).unwrap();

    let mut x = 0.0;
    let mut y = 0.0;
    let mut dir = 1.;

    loop {
        let w = screen_width();
        let h = screen_height();

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        //draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        clear_background(RED);
        draw_texture(&texture, 0., 0., RED);
        // tiled_map.draw_tiles("Tile Layer 1", Rect::new(0.0, 0.0, 320.0, 152.0), None);
        tiled_map.draw_tiles("Tile Layer 1", Rect::new(x, y, 320.0, 152.0), None);
        x += dir;
        if x >= w || x <= 0. {
            dir *= -1.;
            x += dir*3.;
        }
        next_frame().await
    }
}
