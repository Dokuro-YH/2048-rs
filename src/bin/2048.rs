use std::{
    error,
    io::{self, Write},
};

use rim::{Direction, Game};
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};

pub fn main() -> Result<(), Box<dyn error::Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

    let mut game = Game::new();

    ui::draw(&game, &mut stdout);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Left => game.execute(Direction::Left),
            Key::Right => game.execute(Direction::Right),
            Key::Up => game.execute(Direction::Up),
            Key::Down => game.execute(Direction::Down),
            _ => {}
        }

        ui::draw(&game, &mut stdout);
    }

    Ok(())
}

mod ui {
    use rim::Game;
    use std::io::{Stdout, Write};
    use termion::{color, cursor};

    const CONTROLS: &str = "╓──────────┬──CONTROLS────────────╖\n\r\
                            ║  ← ↑ → ↓ | move tiles           ║\n\r\
                            ║ ctrl + c | quit                 ║\n\r\
                            ╚══════════╧══════════════════════╝\n\r";

    pub fn draw(game: &Game, stdout: &mut Stdout) {
        let board = draw_board(game);
        write!(
            stdout,
            "{}score: {}\n{}{}\n{}{}",
            cursor::Goto(1, 2),
            game.score(),
            cursor::Goto(1, 3),
            board,
            CONTROLS,
            cursor::Hide,
        )
        .unwrap();
    }

    fn draw_board(game: &Game) -> String {
        let mut display = String::new();
        let line_break = { "\n\r" };
        display.push_str(&*format!(
            "{b}╔═══════╦═══════╦═══════╦═══════╗{b}",
            b = line_break
        ));

        for i in 0..16 {
            let j = if i == 16 { 0 } else { 15 - i };
            let tile = (game.board >> (j * 4)) & 0xF;
            if tile == 0 {
                display.push_str("║       ");
            } else {
                let tile = 2_u64.pow(tile as u32);
                display.push_str(&*format!(
                    "║{prefix}{color}{tile}{reset} ",
                    prefix = get_spaces_prefix(tile),
                    color = get_color(tile),
                    tile = tile,
                    reset = color::Fg(color::Reset)
                ));
            }
            if i % 4 == 3 {
                display.push_str(&*format!("║{b}", b = line_break));
                if i == 15 {
                    display.push_str(&*format!(
                        "╚═══════╩═══════╩═══════╩═══════╝{b}",
                        b = line_break
                    ));
                } else {
                    display.push_str(&*format!(
                        "╠═══════╬═══════╬═══════╬═══════╣{b}",
                        b = line_break
                    ));
                }
            }
        }
        display
    }

    fn get_spaces_prefix(tile: u64) -> &'static str {
        if tile < 10 {
            "     "
        } else if tile < 100 {
            "    "
        } else if tile < 1000 {
            "   "
        } else if tile < 10000 {
            "  "
        } else {
            " "
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
