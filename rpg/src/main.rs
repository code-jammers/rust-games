use rand::prelude::*;
use ruscii::app::{App, Config, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style, Window};

#[derive(Clone)]
struct Ascii {
    char: char,
    xy: Vec2,
}

#[derive(Clone)]
struct AsciiArt {
    ascii_vec: Vec<Ascii>,
}

impl AsciiArt {
    fn new(ascii_art_string: String, start_x: i32, start_y: i32) -> Self {
        let mut ascii_vec = vec![];
        let mut x = start_x;
        let mut y = start_y;
        for (_, c) in ascii_art_string.chars().enumerate() {
            if c == '\n' {
                x = start_x;
                y += 1;
                continue;
            }
            ascii_vec.push(Ascii {
                char: c,
                xy: Vec2::xy(x, y),
            });
            x += 1;
        }
        return Self { ascii_vec };
    }
    fn draw(&self, pencil: &mut Pencil) {
        for a in &self.ascii_vec {
            pencil.draw_char(a.char, a.xy);
        }
    }
    fn move_by(&mut self, x: i32, y: i32) {
        for (i, _) in self.clone().ascii_vec.iter().enumerate() {
            let xy = self.ascii_vec[i].xy;
            self.ascii_vec[i].xy = Vec2::xy(xy.x + x, xy.y + y);
        }
    }
}

struct GameState {
    player_pos: Vec2,
    player_move: Vec2,
    map_dim: Vec2,
    player_hp: i32,
    player_max_hp: i32,
    player_level: i32,
    player_gold: i32,
    origin: Vec2,
    gold_pos: Vec2,
    monster_pos: Vec2,
    story_part: i32,
    player_xp: i32,
    battle_mode: bool,
}

impl GameState {
    pub fn update(&mut self) {
        let future_pos = self.player_pos + self.player_move;
        self.player_move.clear();

        if future_pos.x < (self.map_dim.x - 1)
            && future_pos.x > 0
            && future_pos.y < (self.map_dim.y + self.origin.y) - 1
            && future_pos.y > self.origin.y
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

fn draw_gold(state: &GameState, pencil: &mut Pencil) {
    pencil
        .set_foreground(Color::Yellow)
        .set_style(Style::Bold)
        .draw_char('o', state.gold_pos);
}

fn draw_monster(state: &GameState, pencil: &mut Pencil) {
    pencil
        .set_foreground(Color::Red)
        .set_style(Style::Bold)
        .draw_char('?', state.monster_pos);
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

fn draw_game_text(state: &GameState, pencil: &mut Pencil) {
    match state.story_part {
        0 => {
            pencil
                .draw_text(
                    "Hello traveler, welcome to our terminal world!",
                    Vec2::xy(0, 0),
                )
                .draw_text("Collect gold to buy a sword!", Vec2::xy(0, 1));
        }
        1 => {
            pencil
                .draw_text(
                    "Wow. Rich boi. You are flush with gold coin!",
                    Vec2::xy(0, 0),
                )
                .draw_text("Press b to buy a sword!", Vec2::xy(0, 1));
        }
        2 => {
            pencil.draw_text(
                "That's a big sword big boi. Let's go kill some monsters!",
                Vec2::xy(0, 0),
            );
        }
        _ => println!("Default case"),
    }
}

fn main() {
    let mut app = App::config(Config::new().fps(20));
    let size = app.window().size();

    let dim_y = (size.y + size.y + size.y) / 4;
    let origin_y = size.y / 4;

    let mut fps_counter = FPSCounter::default();

    println!("size: {:?}", size.x);
    println!("size: {:?}", dim_y);

    let x: i32 = rand::thread_rng().gen_range(1..size.x - 1);
    let y: i32 = rand::thread_rng().gen_range(origin_y + 1..dim_y - 1);

    let mut state = GameState {
        player_pos: Vec2::xy(1, origin_y + 1),
        player_move: Vec2::xy(1, 0),
        map_dim: Vec2::xy(size.x, dim_y),
        origin: Vec2::xy(0, origin_y),
        player_hp: 10,
        player_max_hp: 10,
        player_level: 1,
        player_gold: 0,
        gold_pos: Vec2::xy(x, y),
        monster_pos: Vec2::xy(x, y),
        story_part: 0,
        player_xp: 0,
        battle_mode: false,
    };

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                KeyEvent::Pressed(Key::B) => {
                    if state.story_part == 1 && state.player_gold >= 3 {
                        state.story_part = 2;
                        state.player_gold = state.player_gold - 3;
                        state.player_level = state.player_level + 1;
                        state.player_max_hp = state.player_max_hp + 1;
                        state.player_hp = state.player_max_hp;
                    }
                }
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::H | Key::A => {
                    if !state.battle_mode {
                        state.player_move = Vec2::x(-2);
                    }
                }
                Key::J | Key::S => {
                    if !state.battle_mode {
                        state.player_move = Vec2::y(1);
                    }
                }
                Key::K | Key::W => {
                    if !state.battle_mode {
                        state.player_move = Vec2::y(-1);
                    }
                }
                Key::L | Key::D => {
                    if !state.battle_mode {
                        state.player_move = Vec2::x(2);
                    }
                }
                _ => (),
            }
        }

        fps_counter.update();
        state.update();

        let mut pencil = Pencil::new(window.canvas_mut());

        if state.story_part < 2 {
            if (state.player_pos.x - 1 == state.gold_pos.x
                || state.player_pos.x == state.gold_pos.x
                || state.player_pos.x + 1 == state.gold_pos.x)
                && state.player_pos.y == state.gold_pos.y
            {
                state.player_gold += 1;
                state.gold_pos = Vec2::xy(
                    rand::thread_rng().gen_range(1..size.x - 1),
                    rand::thread_rng().gen_range(origin_y + 1..dim_y - 1),
                );
            }
        }

        if state.story_part >= 2 {
            if (state.player_pos.x - 1 == state.monster_pos.x
                || state.player_pos.x == state.monster_pos.x
                || state.player_pos.x + 1 == state.monster_pos.x)
                && state.player_pos.y == state.monster_pos.y
            {
                state.battle_mode = true;

                let add_player = "
                __*       
                \\/        
                P3        
                ||        
@@@@      _T_  /\\;        
@||||@ .-.[:|:]^\\/         
\\||/ /\\|    \\/          
 E]_|\\/ \\-- ||--/           
      `'  '=:='           
        /'''''\\           
       /'''''''\\          
      []'/'''\\'[]         
      | \\     / |         
      | |     | |
      [__]    [__]
                ";

                let add_monster = "
                /^\\      /^\\
                |  \\    /  |
                ||\\ \\../ /||
                )'        `(
               ,;`w,    ,w';,
               ;,  ) __ (  ,;
                ;  \\(\\/)/  ;;
               ;|  |vwwv|    ``-...
                ;  `lwwl'   ;      ```''-.
               ;| ; `' ; ;              `.
                ;         ,   ,          , |
                '  ;      ;   l    .     | |
                ;    ,  ,    |,-,._|      \\;
                 ;  ; `' ;   '    \\ `\\     \\;
                 |  |    |  |     |   |    |;
                 |  ;    ;  |      \\   \\   (;
                 | |      | l       | | \\  |
                 | |      | |       | |  ) |
                 | |      | ;       | |  | |
                 ; ,      : ,      ,_.'  | |
                :__'      | |           ,_.'
                         `--'
    ";

                let mut add_wolf_art =
                    AsciiArt::new(add_monster.to_string(), state.map_dim.x - 50, dim_y - 17);
                let mut add_player_art = AsciiArt::new(add_player.to_string(), 10, dim_y - 15);
                // state.player_gold += 1;
                // state.player_xp += 1;
                // state.player_hp -= 1;
                // if state.player_xp >= 5 {
                //     // Level up. Reset player health
                //     state.player_level += 1;
                //     state.player_max_hp += 1;
                //     state.player_xp = 0;
                //     state.player_hp = state.player_max_hp;
                // }
                // state.monster_pos = Vec2::xy(
                //     rand::thread_rng().gen_range(1..size.x - 1),
                //     rand::thread_rng().gen_range(origin_y + 1..dim_y - 1),
                // );
                add_wolf_art.draw(&mut pencil);
                add_player_art.draw(&mut pencil);
            }
        }

        if state.player_gold >= 3 && state.story_part == 0 {
            state.story_part = 1;
        }

        draw_hud(&state, &mut pencil);
        draw_game_text(&state, &mut pencil);
        pencil.set_foreground(Color::Grey);
        pencil.draw_rect(
            &RectCharset::simple_lines(),
            Vec2::xy(0, origin_y),
            state.map_dim,
        );
        if (!state.battle_mode) {
            draw_character(&state, &mut pencil);
            if state.story_part < 2 {
                draw_gold(&state, &mut pencil);
            } else {
                draw_monster(&state, &mut pencil);
            }
        }
    });
}
