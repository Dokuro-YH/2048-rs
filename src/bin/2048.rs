use std::{
    error,
    io::{self, Write},
};

use rim::{Direction, Game};
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};

pub fn main() -> Result<(), Box<dyn error::Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;

    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;

    // let storage = InMemoryStorage::new();
    let mut game = Game::new();

    ui::draw(&game, &mut stdout)?;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Ctrl('r') => game = Game::new(),
            Key::Left => game.execute(Direction::Left),
            Key::Right => game.execute(Direction::Right),
            Key::Up => game.execute(Direction::Up),
            Key::Down => game.execute(Direction::Down),
            _ => {}
        }

        ui::draw(&game, &mut stdout)?;
    }

    Ok(())
}

mod ui {
    use rim::{Game, Storage};
    use std::io::{Stdout, Write};
    use termion::{color, cursor};

    const CONTROLS: &str = "╔══════════╦══CONTROLS══════════╗\n\r\
                            ║ ← ↑ → ↓  ║ move tiles         ║\n\r\
                            ║ ctrl + c ║ quit               ║\n\r\
                            ║ ctrl + r ║ restart            ║\n\r\
                            ╚══════════╩════════════════════╝\n\r";

    const GAME_OVER: &str = "╔═══════════════════════════════╗\n\r\
                             ║                               ║\n\r\
                             ║                               ║\n\r\
                             ║                               ║\n\r\
                             ║           Game Over!          ║\n\r\
                             ║                               ║\n\r\
                             ║                               ║\n\r\
                             ║                               ║\n\r\
                             ╚═══════════════════════════════╝\n\r";

    pub fn draw<S: Storage>(game: &Game<S>, stdout: &mut Stdout) -> std::io::Result<()> {
        let score = format!("Score: {}, Best: {}", game.score(), game.best());
        write!(
            stdout,
            "{}{:<33}",
            cursor::Goto(1, 1),
            score,
        )?;

        if game.game_over() {
            write!(stdout, "{}{grid}", cursor::Goto(1, 2), grid = GAME_OVER)?;
        } else {
            write!(
                stdout,
                "{}{grid}",
                cursor::Goto(1, 2),
                grid = draw_grid(game)
            )?;
        }

        write!(
            stdout,
            "{}{controls}",
            cursor::Goto(1, 11),
            controls = CONTROLS
        )?;

        write!(stdout, "{}", cursor::Hide)?;

        Ok(())
    }

    fn draw_grid<S: Storage>(game: &Game<S>) -> String {
        let mut display = String::new();
        let line_break = { "\n\r" };
        display.push_str(&*format!(
            "╔═══════╦═══════╦═══════╦═══════╗{b}",
            b = line_break
        ));

        for (row_num, row) in game.grid().iter().enumerate() {
            for (col_num, col) in row.iter().enumerate() {
                let tile = get_tile(*col);

                if tile == 0 {
                    display.push_str("║       ");
                } else {
                    display.push_str(&*format!(
                        "║{prefix}{color}{tile}{reset}",
                        prefix = get_spaces_prefix(tile),
                        color = get_color(tile),
                        tile = tile,
                        reset = color::Fg(color::Reset),
                    ));
                }

                if col_num == 3 {
                    display.push_str(&*format!("║{b}", b = line_break));
                }
            }
            if row_num == 3 {
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

        display
    }

    fn get_spaces_prefix(tile: u64) -> &'static str {
        if tile < 10 {
            "      "
        } else if tile < 100 {
            "     "
        } else if tile < 1000 {
            "    "
        } else if tile < 10000 {
            "   "
        } else {
            "  "
        }
    }

    fn get_tile(val: u8) -> u64 {
        match val {
            0 => 0,
            1 => 2,
            2 => 4,
            x => 2_u64.pow((x + 1) as u32),
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
