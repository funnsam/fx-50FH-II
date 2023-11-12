#[derive(Debug)]
pub struct Calculator {
    mode: Mode,
    menu: Option<(Menu, usize)>,
}

#[derive(Debug)]
enum Mode {
    Computation,
    Complex,
    Base(Base),
    SingleStat,
    PairedStat,
    Program
}

#[derive(Debug)]
enum Base {
    Binary, Octal, Decimal, Hexadecimal
}

#[derive(Debug)]
enum Menu {
    ModeSelect
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            mode: Mode::Complex,
            menu: None,
        }
    }

    pub fn on_key_event(&mut self, ke: crossterm::event::KeyEvent) {
        use crossterm::event::*;
        match ke {
            KeyEvent { code: KeyCode::Char('4'), kind: KeyEventKind::Press, state: KeyEventState::NONE, .. } => self.toggle_menu(),
            _ => (),
        }
    }

    pub fn tick(&mut self) {
    }

    pub fn toggle_menu(&mut self) {
        self.menu = match self.menu {
            None => Some((Menu::ModeSelect, 0)),
            Some((Menu::ModeSelect, 0)) => Some((Menu::ModeSelect, 1)),
            Some((Menu::ModeSelect, 1)) => None,
            _ => return,
        }
    }

    pub fn get_display(&self) -> (String, String, String) {
        let mut stat = String::new();
        let mut top = String::new();
        let mut bot = String::new();

        stat += &format!(
         // "SAhMSR CMPLX_SDREGPROG
         //            r∠θR⇔I",
            "       {}       ",
            self.mode.status_name()
        );

        match self.menu {
            None => {
                top += " 3+2(3-2)";
                bot += "                5.";
            },
            Some((Menu::ModeSelect, 0)) => {
                top += "←COMP CMPLX BASE→";
                bot += "  1    2     3   ";
            },
            Some((Menu::ModeSelect, 1)) => {
                top += "← SD  REG  PRGM →";
                bot += "  4    5     6   ";
            },
            Some((Menu::ModeSelect, _)) => unreachable!(),
        }

        (stat, top, bot)
    }
}

impl Mode {
    pub fn status_name(&self) -> &'static str {
        match self {
            Mode::Computation               => "               ",
            Mode::Complex                   => "CMPLX          ",
            Mode::Base(Base::Binary)        => "     b         ",
            Mode::Base(Base::Octal)         => "     o         ",
            Mode::Base(Base::Decimal)       => "     d         ",
            Mode::Base(Base::Hexadecimal)   => "     h         ",
            Mode::SingleStat                => "      SD       ",
            Mode::PairedStat                => "        REG    ",
            Mode::Program                   => "           PROG",
        }
    }
}
