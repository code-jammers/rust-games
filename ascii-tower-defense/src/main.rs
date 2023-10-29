use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;

#[derive(Clone)]
struct Ascii {
    char: char,
    xy: Vec2
}

#[derive(Clone)]
struct AsciiArt {
    ascii_vec: Vec<Ascii>
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
    fn move_by(&mut self, x: i32, y: i32) {
        for (i,_) in self.clone().ascii_vec.iter().enumerate() {
            let xy = self.ascii_vec[i].xy;
            self.ascii_vec[i].xy = Vec2::xy(xy.x+x, xy.y+y);
        }
    }
}

fn main() {
    //let mut key_events: Vec<KeyEvent> = Vec::new();
    let mut app = App::default();

    let bullet_str = "-->";

    let add_tower = 
"_________
|       |
 |     |
 |  +  |
 |     |
|_______|
";

    let mut add_tower_art = AsciiArt::new(add_tower.to_string(), 30, 4);

    // 24
    let road =
"|                |
|                |
|                |
|                |
|                |
|                |
";

    let road1 = AsciiArt::new(road.to_string()+&road.to_string()[..]+&road.to_string()[..]+&road.to_string()[..], 45, 0);
    /*let road2 = AsciiArt::new(road.to_string(), 45, 6);
    let road3 = AsciiArt::new(road.to_string(), 45, 13);
    let road4 = AsciiArt::new(road.to_string(), 45, 9);*/

    let monster_guy = 
"      __
  ___(**)__
 (**)    |_)
  | |  | |`
  \\ /  \\ /
";
    let mut monster = AsciiArt::new(monster_guy.to_string(), 50, 0);

    let frame_interval = 30;
    let mut frame = 1;

    let mut towers: Vec<AsciiArt> = vec![];
    let mut bullets: Vec<AsciiArt> =vec![];
    //let mut bullet: &mut AsciiArt;

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            if let KeyEvent::Pressed(Key::Q) = key_event {
                app_state.stop();
            }
        }
        for key_down in app_state.keyboard().get_keys_down() {
            //key_events.push(*key_event);
            match key_down {
                Key::Left => add_tower_art.move_by(-1, 0),
                Key::Right => add_tower_art.move_by(1, 0),
                Key::Up => add_tower_art.move_by(0, -1),
                Key::Down => add_tower_art.move_by(0, 1),
                Key::Enter => towers.push(AsciiArt { ascii_vec: add_tower_art.ascii_vec.clone() }),
                _ => (),
            }
        }
        let mut pencil = Pencil::new(window.canvas_mut());
        road1.draw(&mut pencil);
        /*road2.draw(&mut pencil);
        road3.draw(&mut pencil);
        road4.draw(&mut pencil);*/
        add_tower_art.draw(&mut pencil);
        monster.draw(&mut pencil);

        for t in towers.clone() {
            t.draw(&mut pencil);
            let x = t.ascii_vec[0].xy.x+5;
            let y = t.ascii_vec[0].xy.y +2;
            let bullet: AsciiArt = AsciiArt::new(bullet_str.to_string(), x+4, y);
            
            //let bullet = &mut AsciiArt::new(bullet_str.to_string(), x+4, y);
            if frame % frame_interval == 0 {
                bullets.push(bullet);
            }
        }
        
        for i in 0..bullets.len() {
            bullets[i].move_by(1, 0);
            bullets[i].draw(&mut pencil);
        }
        //for (_, &mut b) in bullets.iter().enumerate() {
        //    b.clone().draw(&mut pencil);
        //    b.move_by(1, 0);
        //}
        //pencil.draw_text("Hello, world", Vec2::xy(0, 0));
        if frame % frame_interval == 0 {
            monster.move_by(0, 1);
        }
        frame += 1;
        
    });
}
