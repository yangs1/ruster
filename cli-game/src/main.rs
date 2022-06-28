mod game;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyEvent, KeyCode},
    execute,queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode},
    Result,
};
use std::{io::{stdout, Write}, thread::sleep};
use std::time::Duration;
use std::{thread, time};

fn  main () -> Result<()> {
	let mut stdout = stdout();

	// 开启输入模式
	enable_raw_mode()?;

	execute!(
        stdout,
        EnterAlternateScreen,
        SetForegroundColor(Color::Magenta),
		Hide
    ).unwrap();

	let mut game = game::Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);

	let mut color_map = [
		Color::Reset, Color::Black,Color::DarkGrey,Color::Red,Color::DarkRed,Color::Green,
		Color::DarkGreen,Color::Yellow,Color::DarkYellow,Color::Blue,Color::DarkBlue,Color::Magenta,
		Color::DarkMagenta,Color::Cyan,Color::DarkCyan,Color::White,Color::Grey].to_vec();
		
	loop {
		queue!(stdout, Clear(ClearType::All))?;
		let mut i = 0;
		while let Some(line) = game.row_as_string(i) {
			queue!(stdout, MoveTo(0, i as u16), Print(line))?;
			i += 1;
		}

		queue!(stdout, MoveTo(0, (i + 1) as u16), Print("Press Esc to exit..."))?;
		stdout.flush()?;
		game.tick();


		if poll(Duration::from_millis(500))? {
			if let Event::Key(KeyEvent { code, .. }) = read()? {
				match code {
					KeyCode::Esc => {
						break;
					},
					KeyCode::Tab => {
						let tmp_color = color_map.pop().unwrap();
						execute!(stdout, SetForegroundColor(tmp_color))?;
						color_map.insert(0, tmp_color);
					}
					_ => {}
				}
			}
		}
	}

	execute!(stdout, ResetColor, Show, LeaveAlternateScreen)?;

	Ok(())

}





/*
fn main() -> Result<()> {

	let mut game = game::Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    print!("{}", game);

	game.tick();
	let line = game.row_as_string(2);
	println!("{:?}", line);

	let mut stdout = stdout();

    let mut game = game::Universe::new(5, 5);
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    execute!(
        stdout,
        EnterAlternateScreen,
        SetForegroundColor(Color::Magenta),
        Hide
    )?;

    loop {
		if poll(Duration::from_millis(500))? {
			if let Event::Key(KeyEvent { code, .. }) = read()? {
				match code {
					KeyCode::Esc => {
						break;
					}
					_ => {}
				}
			}
		} else {
			queue!(stdout, Clear(ClearType::All))?;
			let mut i = 0;
			while let Some(line) = game.row_as_string(i) {
				queue!(stdout, MoveTo(0, i as u16), Print(line))?;
				i += 1;
			}
	
			queue!(
				stdout,
				MoveTo(0, (i + 1) as u16),
				Print("Press Esc to exit...")
			)?;
			stdout.flush()?;
			game.tick();
		}
	}
    execute!(stdout, ResetColor, Show, LeaveAlternateScreen)?;


    Ok(())
}

*/