#[macro_use]
extern crate lazy_static;

use crate::game::{Game, State};
use crate::game_ui::GameUI;
use crate::words_lib::WordsLib;
use anyhow::Result;
use std::io;
use std::io::{stdout, Write};
use termion::screen::AlternateScreen;

mod game;
mod game_ui;
mod words_lib;

lazy_static! {
    static ref WORDS_LIB: WordsLib =
        WordsLib::load("./assets/words.data", "./assets/popular-words.data")
            .expect("Failed to load words lib");
}

fn main() -> Result<()> {
    let mut game = Game::new(6)?;
    let mut screen = AlternateScreen::from(stdout());
    let mut err: Option<anyhow::Error> = None;
    let mut buffer = String::new();
    loop {
        write!(screen, "{}", termion::cursor::Save)?;
        write!(screen, "{}\n", GameUI::display_header(&game))?;
        write!(screen, "{}\n", GameUI::display_board(&game))?;
        write!(screen, "{}", termion::clear::AfterCursor)?;
        write!(screen, "{}", termion::cursor::BlinkingUnderline)?;
        if let Some(error) = &err {
            write!(
                screen,
                "{}{}{}\n",
                termion::color::Fg(termion::color::LightRed),
                error,
                termion::color::Fg(termion::color::Reset)
            )?;
        }
        screen.flush()?;
        match game.game_state() {
            State::Playing => {
                io::stdin().read_line(&mut buffer)?;
                if let Err(error) = game.guess(buffer.as_str()) {
                    err = Some(error);
                } else {
                    err = None;
                }
                buffer.clear();
            }
            State::Lose => {
                let word = game.get_word_after_game_end()?;
                write!(screen, "Lose. Correct answer is: \"{}\".\n", word)?;
                break;
            }
            State::Win => {
                write!(
                    screen,
                    "Correct. You guessed the word after {} turns.\n",
                    game.turn()
                )?;
                break;
            }
        }
        screen.flush()?;
        write!(screen, "{}", termion::cursor::Restore)?;
    }
    write!(screen, "Press ENTER to exit.")?;
    screen.flush()?;
    io::stdin().read_line(&mut buffer)?;
    Ok(())
}
