use ruscii::app::{App, Config, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style, Window};

struct GameState {
    player_pos: Vec2,
    player_move: Vec2,
    map_dim: Vec2,
    player_hp: i32,
    player_max_hp: i32,
    player_level: i32,
    player_gold: i32,
    origin: Vec2,
}

impl GameState {
    pub fn update(&mut self) {
        let future_pos = self.player_pos + self.player_move;
        self.player_move.clear();

        if future_pos.x < (self.map_dim.x - 0)
            && future_pos.x > 0
            && future_pos.y <= (self.map_dim.y + self.origin.y) - 1
            && future_pos.y >= self.origin.y
        {
            self.player_pos = future_pos;
        }
    }
}

fn draw_character(state: &GameState, pencil: &mut Pencil) {
    pencil
        .set_foreground(Color::Blue)
        .set_style(Style::Bold)
        .draw_char('X', state.player_pos);
}

fn draw_hud(state: &GameState, pencil: &mut Pencil) {
    let middle_screen_position_x = (state.map_dim.x / 2) - 18;
    pencil
        .draw_text(
            "          Terminal RPG            ",
            Vec2::xy(middle_screen_position_x, 1),
        )
        .draw_text(
            &format!(
                "Level: {} - Gold: {} - Health: {}/{}",
                state.player_level, state.player_gold, state.player_hp, state.player_max_hp
            ),
            Vec2::xy(middle_screen_position_x, 2),
        )
        .draw_text(
            "**********************************",
            Vec2::xy(middle_screen_position_x, 3),
        );
}

fn main() {
    let mut app = App::config(Config::new().fps(20));
    let size = app.window().size();

    let dim_y = (size.y + size.y + size.y) / 4;
    let origin_y = size.y / 4;

    let mut fps_counter = FPSCounter::default();
    let mut state = GameState {
        player_pos: Vec2::xy(0, origin_y),
        player_move: Vec2::xy(1, 0),
        map_dim: Vec2::xy(size.x, dim_y),
        origin: Vec2::xy(0, origin_y),
        player_hp: 10,
        player_max_hp: 10,
        player_level: 1,
        player_gold: 0,
    };

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::H | Key::A => state.player_move = Vec2::x(-2),
                Key::J | Key::S => state.player_move = Vec2::y(1),
                Key::K | Key::W => state.player_move = Vec2::y(-1),
                Key::L | Key::D => state.player_move = Vec2::x(2),
                _ => (),
            }
        }

        fps_counter.update();
        state.update();

        let mut pencil = Pencil::new(window.canvas_mut());

        draw_hud(&state, &mut pencil);
        pencil.set_foreground(Color::Grey);
        pencil.draw_rect(
            &RectCharset::simple_lines(),
            Vec2::xy(0, origin_y),
            state.map_dim,
        );
        draw_character(&state, &mut pencil);
    });
}
