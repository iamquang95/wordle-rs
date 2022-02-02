use crate::game::JudgedChar;
use crate::Game;

use termion::color;

pub struct GameUI;

impl GameUI {
    pub fn display_board(game: &Game) -> String {
        let result = game.current_result();
        let mut rows = result
            .judged_guesses
            .iter()
            .map(|res| {
                res.guess.clone().chars().zip(&res.result).fold(
                    String::new(),
                    |mut row, (ch, judge)| {
                        row = row + &GameUI::styled_judged_char(ch, judge);
                        row
                    },
                )
            })
            .collect::<Vec<String>>();
        if rows.len() < game.num_guesses {
            let empty_row = std::iter::repeat(GameUI::styled_char(
                ' ',
                Box::new(color::LightWhite),
                Box::new(color::LightWhite),
            ))
            .take(game.word_len())
            .collect::<String>();
            rows.resize(game.num_guesses, empty_row);
        }
        while rows.len() < game.num_guesses {}
        rows.join("\n")
    }

    fn styled_judged_char(ch: char, judge: &JudgedChar) -> String {
        let (bg, fg): (Box<dyn color::Color>, Box<dyn color::Color>) = match judge {
            JudgedChar::Wrong => (Box::new(color::Black), Box::new(color::White)),
            JudgedChar::WrongPlace => (Box::new(color::LightYellow), Box::new(color::Black)),
            JudgedChar::Correct => (Box::new(color::LightGreen), Box::new(color::Black)),
        };
        GameUI::styled_char(ch, bg, fg)
    }

    fn styled_char(ch: char, bg: Box<dyn color::Color>, fg: Box<dyn color::Color>) -> String {
        format!(
            "{}{} {} {}{}",
            color::Bg(bg.as_ref()),
            color::Fg(fg.as_ref()),
            ch,
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        )
    }
}
