use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;

struct Ascii {
    char: char,
    xy: Vec2
}
struct AsciiArt {
    ascii_vec: Vec<Ascii>
}

impl AsciiArt {
    fn new(ascii_art_string: String) -> Self {
        let mut ascii_vec = vec![];
        let mut x = 0;
        let mut y = 0;
        for (_, c) in ascii_art_string.chars().enumerate() {
            if c == '\n' {
                x = 0;
                y += 1;
                continue;
            }
            ascii_vec.push(Ascii { char: c, xy: Vec2::xy(x,y) });
            x += 1;
        }
        return Self { ascii_vec };
    }
    fn draw(&self, pencil: &mut Pencil) {
        for a in &self.ascii_vec {
            pencil.draw_char(a.char, a.xy);
        }
    }
}

fn main() {
    let mut key_events: Vec<KeyEvent> = Vec::new();
    let mut app = App::default();

    let monster_guy = "
      __
  ___(**)__
 (**)    |_)
  | |  | |`
  \\ /  \\ /
";
    let monster = AsciiArt::new(monster_guy.to_string());

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            key_events.push(*key_event);
            if let KeyEvent::Pressed(Key::Q) = key_event {
                app_state.stop();
            }
        }
        let mut pencil = Pencil::new(window.canvas_mut());
        monster.draw(&mut pencil);
        //pencil.draw_text("Hello, world", Vec2::xy(0, 0));
    });
}
