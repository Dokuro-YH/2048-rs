use std::{
    error,
    io::{self, Write},
};

use rim::{Direction, Game};
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};

pub fn main() -> Result<(), Box<dyn error::Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;

    write!(
        stdout,
        "{}{}{}",
        clear::All,
        cursor::Goto(1, 1),
        cursor::Hide
    )?;

    let mut game = Game::new();

    ui::draw(&game, &mut stdout)?;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Ctrl('r') => game.restart(),
            Key::Left => game.execute(Direction::Left),
            Key::Right => game.execute(Direction::Right),
            Key::Up => game.execute(Direction::Up),
            Key::Down => game.execute(Direction::Down),
            _ => {}
        }

        ui::draw(&game, &mut stdout)?;
    }

    write!(stdout, "{}", cursor::Show)?;
    Ok(())
}

mod ui {
    use rim::Game;
    use std::io::{Stdout, Write};
    use termion::{color, cursor};

    const LINE_BREAK: &str = "\n\r";

    const CONTROLS: &str = "╔════════════╦══CONTROLS════════╗\n\r\
                            ║  ← ↑ → ↓   ║ move tiles       ║\n\r\
                            ║  ctrl + r  ║ restart          ║\n\r\
                            ║  ctrl + c  ║ quit             ║\n\r\
                            ╚════════════╩══════════════════╝\n\r";

    pub fn draw(game: &Game, stdout: &mut Stdout) -> std::io::Result<()> {
        write!(stdout, "{}{}", cursor::Goto(1, 1), draw_title(game))?;
        write!(stdout, "{}{}", cursor::Goto(1, 5), draw_grid(game))?;
        write!(stdout, "{}{}", cursor::Goto(1, 14), CONTROLS)?;
        Ok(())
    }

    fn draw_title(game: &Game) -> String {
        let mut display = String::new();
        let title = if game.game_over() {
            String::from("Game Over!")
        } else {
            String::from("2048 Game")
        };
        let score = format!("Score: {}, Best: {}", game.score(), "0");
        display.push_str(&*format!("╔═══════════════════════════════╗{}", LINE_BREAK));
        display.push_str(&*format!("║{:^31}║{}", title, LINE_BREAK));
        display.push_str(&*format!("║{:^31}║{}", score, LINE_BREAK));
        display.push_str(&*format!("╚═══════════════════════════════╝{}", LINE_BREAK));
        display
    }

    fn draw_grid(game: &Game) -> String {
        let mut display = String::new();
        let mut row_num = 1;
        let mut col_num = 1;

        display.push_str(&*format!("╔═══════╦═══════╦═══════╦═══════╗{}", LINE_BREAK));
        for row in game.grid().iter() {
            for col in row.iter() {
                let tile = get_tile(*col);
                let val = if tile == 0 {
                    String::new()
                } else {
                    tile.to_string()
                };

                display.push_str(&*format!(
                    "║{}{:^7}{}",
                    get_color(tile),
                    val,
                    color::Fg(color::Reset),
                ));

                if col_num % 4 == 0 {
                    display.push_str(&*format!("║{}", LINE_BREAK));
                }

                col_num += 1;
            }

            if row_num % 4 == 0 {
                display.push_str(&*format!("╚═══════╩═══════╩═══════╩═══════╝{}", LINE_BREAK));
            } else {
                display.push_str(&*format!("╠═══════╬═══════╬═══════╬═══════╣{}", LINE_BREAK));
            }

            row_num += 1;
        }

        display
    }

    fn get_tile(val: u8) -> u64 {
        match val {
            0 => 0,
            1 => 2,
            x => 2_u64.pow(x as u32),
        }
    }

    fn get_color(tile: u64) -> color::Fg<color::Rgb> {
        match tile {
            2 => color::Fg(color::Rgb(238, 228, 218)),
            4 => color::Fg(color::Rgb(237, 224, 200)),
            8 => color::Fg(color::Rgb(242, 177, 121)),
            16 => color::Fg(color::Rgb(245, 149, 99)),
            32 => color::Fg(color::Rgb(246, 124, 95)),
            64 => color::Fg(color::Rgb(246, 94, 59)),
            128 => color::Fg(color::Rgb(237, 207, 114)),
            256 => color::Fg(color::Rgb(237, 204, 97)),
            512 => color::Fg(color::Rgb(237, 200, 80)),
            1024 => color::Fg(color::Rgb(237, 197, 63)),
            2048 => color::Fg(color::Rgb(237, 194, 46)),
            4096 => color::Fg(color::Rgb(129, 214, 154)),
            8192 => color::Fg(color::Rgb(129, 214, 154)),
            16384 => color::Fg(color::Rgb(129, 214, 154)),
            32768 => color::Fg(color::Rgb(129, 214, 154)),
            65536 => color::Fg(color::Rgb(129, 214, 154)),
            _ => color::Fg(color::Rgb(129, 214, 154)),
        }
    }
}
