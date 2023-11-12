use std::{io::{Write, stdout}, time::*};
use crossterm::{*, style::{Color, Stylize}, event::*};

mod calculator;
use calculator::Calculator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0))?;
    terminal::enable_raw_mode()?;

    queue!(stdout, style::PrintStyledContent(
        "Virtual fx-50FH II"
            .italic()
            .with(Color::Black)
            .on(Color::Grey)
    ))?;

    stdout.flush()?;

    let mut calc = Calculator::new();

    'main_loop: loop {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(KeyEvent { code: KeyCode::Esc, kind: KeyEventKind::Press, .. }) => break 'main_loop,
                Event::Key(ke) => calc.pretick(Some(ke)),
                _ => ()
            }
        } else {
            calc.pretick(None)
        }

        calc.tick();

        let (stat, top, bot, cursor) = calc.get_display();
        queue!(
            stdout,

            cursor::MoveTo(0, 2),
            terminal::Clear(terminal::ClearType::FromCursorDown),

            style::Print(stat),

            cursor::MoveTo(0, 3),
            style::PrintStyledContent(
                top .with(Color::White)
            ),

            cursor::MoveTo(0, 4),
            style::PrintStyledContent(
                bot .bold()
                    .with(Color::White)
            ),
        )?;

        match cursor {
            Some((x, false)) => queue!(
                stdout,

                cursor::MoveTo(x as u16 + 1, 3),
                cursor::SetCursorStyle::BlinkingBar,
            )?,
            Some((x, true)) => queue!(
                stdout,

                cursor::MoveTo(x as u16 + 1, 3),
                cursor::SetCursorStyle::BlinkingUnderScore,
            )?,
            None => queue!(stdout, cursor::Hide)?,
        }

        stdout.flush()?;
    }
    
    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show, cursor::SetCursorStyle::DefaultUserShape)?;
    Ok(())
}
