#[derive(Debug)]
pub struct Calculator {
    mode: Mode,
    menu: Option<(Menu, usize)>,

    pending_key: Option<Key>,

    cursor_at: usize,
    insert_mode: bool,
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

#[derive(Debug)]
enum Key {
    Shift, Alpha, Up, Right, Mode,
    Prog, Fmla, Left, Down, PowNegOne, Cubed,
    Fraction, SquareRoot, Squared, Power, Log, Ln,
    Negative, Unknown, Hyp, Sin, Cos, Tan,
    Rcl, Eng, BracketStart, BracketEnd, Comma, MPlus
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            mode: Mode::Complex,
            menu: None,

            pending_key: None,

            cursor_at: 0,
            insert_mode: false,
        }
    }

    pub fn pretick(&mut self, ke: Option<crossterm::event::KeyEvent>) {
        use crossterm::event::*;

        macro_rules! key_map {
            ($($($state: ident $og: pat_param)|+ => $k: expr), * $(,)?) => {
                match ke {
                    $(
                        $(Some(KeyEvent { code: $og, kind: KeyEventKind::Press, state: KeyEventState::$state, .. }))|+ => {
                            self.pending_key = Some($k)
                        },
                    )*
                    _ => self.pending_key = None,
                }
            };
        }

        key_map!(
            NONE KeyCode::Char('`') => Key::Shift,
            NONE KeyCode::Char('1') => Key::Alpha,
            NONE KeyCode::Char('2') | NONE KeyCode::Up    => Key::Up,
            NONE KeyCode::Char('3') | NONE KeyCode::Right => Key::Right,
            NONE KeyCode::Char('4') => Key::Mode,
            NONE KeyCode::Char('q') => Key::Prog,
            NONE KeyCode::Char('w') => Key::Fmla,
            NONE KeyCode::Char('e') => Key::Left,
            NONE KeyCode::Char('r') => Key::Down,
            NONE KeyCode::Char('t') => Key::PowNegOne,
            NONE KeyCode::Char('y') => Key::Cubed,
            NONE KeyCode::Char('u') => Key::Rcl,
        );
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

    pub fn get_display(&self) -> (String, String, String, Option<(usize, bool)>) {
        let mut stat = String::new();
        let mut top = String::new();
        let mut bot = String::new();
        let mut cursor = None;

        stat += &format!(
         // "SAhMSR CMPLX_SDREGPROG
         //            r∠θR⇔I",
            "       {}       {:?}",
            self.mode.status_name(), self.pending_key
        );

        match self.menu {
            Some((Menu::ModeSelect, 0)) => {
                top += "←COMP CMPLX BASE→";
                bot += "  1    2     3   ";
            },
            Some((Menu::ModeSelect, 1)) => {
                top += "← SD  REG  PRGM →";
                bot += "  4    5     6   ";
            },
            Some((Menu::ModeSelect, _)) => unreachable!(),
            None => {
                top += " 3+2(3-2)";
                bot += "                5.";
                cursor = Some((self.cursor_at, self.insert_mode));
            },
        }

        (stat, top, bot, cursor)
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
