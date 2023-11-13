#[derive(Debug)]
pub struct Calculator {
    mode: Mode,
    menu: Option<(Menu, usize)>,

    pending_key: Option<Key>,
    modifier_key: Option<KeyModifier>,

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
    Negative, Base60, Hyp, Sin, Cos, Tan,
    Rcl, Eng, BracketStart, BracketEnd, Comma, MPlus,
    Del, Ac, Add, Subtract, Multiply, Divide,
    _0, _1, _2, _3, _4, _5, _6, _7, _8, _9,
    Dot, Exp, Ans, Exe
}

#[derive(Debug)]
enum KeyModifier {
    Shift, ShiftHyp, Alpha, RCL, Sto, Hyp
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            mode: Mode::Complex,
            menu: None,

            pending_key: None,
            modifier_key: None,

            cursor_at: 0,
            insert_mode: false,
        }
    }

    pub fn pretick(&mut self, ke: Option<crossterm::event::KeyEvent>) {
        use crossterm::event::*;

        macro_rules! key_map {
            ($($($og: pat_param)|+ => $k: expr), * $(,)?) => {
                match ke {
                    $(
                        $(Some(KeyEvent { code: $og, kind: KeyEventKind::Press, .. }))|+ => {
                            self.pending_key = Some($k)
                        },
                    )*
                    _ => self.pending_key = None,
                }
            };
        }

        key_map!(
            KeyCode::Char('l') => Key::Add,
            KeyCode::Char('-') => Key::Subtract,
            KeyCode::Char(';') => Key::Multiply,

            KeyCode::Char('`') => Key::Shift,
            KeyCode::Char('0') => Key::_0,
            KeyCode::Char('1') => Key::_1,
            KeyCode::Char('2') => Key::_2,
            KeyCode::Char('3') => Key::_3,
            KeyCode::Char('4') => Key::_4,
            KeyCode::Char('5') => Key::_5,
            KeyCode::Char('6') => Key::_6,
            KeyCode::Char('7') => Key::_7,
            KeyCode::Char('8') => Key::_8,
            KeyCode::Char('9') => Key::_9,
            KeyCode::Char('=') => Key::Alpha,
            KeyCode::Enter     => Key::Exe,
            KeyCode::Backspace => Key::Del,

            KeyCode::Char('q') => Key::Prog,
            KeyCode::Char('w') => Key::Fmla,
            KeyCode::Char('t') => Key::PowNegOne,
            KeyCode::Char('y') => Key::Cubed,
            KeyCode::Char('u') => Key::Rcl,
            KeyCode::Char('i') => Key::Eng,
            KeyCode::Char('o') => Key::BracketStart,
            KeyCode::Char('p') => Key::BracketEnd,
            KeyCode::Char('[') => Key::Comma,
            KeyCode::Char(']') => Key::MPlus,
            KeyCode::Char('\\') => Key::Mode,

            KeyCode::Char('a') => Key::Fraction,
            KeyCode::Char('s') => Key::SquareRoot,
            KeyCode::Char('d') => Key::Squared,
            KeyCode::Char('f') => Key::Power,
            KeyCode::Char('g') => Key::Log,
            KeyCode::Char('h') => Key::Ln,
            KeyCode::Char('k') => Key::Ac,

            KeyCode::Char('z') => Key::Negative,
            KeyCode::Char('x') => Key::Base60,
            KeyCode::Char('c') => Key::Hyp,
            KeyCode::Char('v') => Key::Sin,
            KeyCode::Char('b') => Key::Cos,
            KeyCode::Char('n') => Key::Tan,
            KeyCode::Char('m') => Key::Ans,
            KeyCode::Char(',') => Key::Exp,
            KeyCode::Char('.') => Key::Dot,
            KeyCode::Char('/') => Key::Divide,

            KeyCode::Left  => Key::Left,
            KeyCode::Down  => Key::Down,
            KeyCode::Up    => Key::Up,
            KeyCode::Right => Key::Right,
        );
    }

    pub fn tick(&mut self) {
        match (&self.modifier_key, &self.pending_key, &self.menu, &self.mode) {
            (Some(KeyModifier::Shift | KeyModifier::ShiftHyp), Some(Key::Mode), None, _) => self.modifier_key = None,
            (Some(KeyModifier::Shift | KeyModifier::ShiftHyp), Some(Key::Mode), Some(_), _) => self.modifier_key = None,
            (_, Some(Key::Mode), menu, _) => self.menu = match menu {
                None => Some((Menu::ModeSelect, 0)),
                Some((Menu::ModeSelect, 0)) => Some((Menu::ModeSelect, 1)),
                Some((Menu::ModeSelect, 1)) => None,
                _ => return,
            },
            (Some(KeyModifier::Shift), Some(Key::Shift), _, _) => self.modifier_key = None,
            (Some(KeyModifier::Hyp), Some(Key::Shift), _, _) => self.modifier_key = Some(KeyModifier::ShiftHyp),
            (Some(KeyModifier::ShiftHyp), Some(Key::Shift), _, _) => self.modifier_key = Some(KeyModifier::Hyp),
            (_, Some(Key::Shift), _, _) => self.modifier_key = Some(KeyModifier::Shift),

            (Some(KeyModifier::Hyp), Some(Key::Hyp), _, _) => self.modifier_key = None,
            (Some(KeyModifier::Shift), Some(Key::Hyp), _, _) => self.modifier_key = Some(KeyModifier::ShiftHyp),
            (Some(KeyModifier::ShiftHyp), Some(Key::Hyp), _, _) => self.modifier_key = Some(KeyModifier::Shift),
            (_, Some(Key::Hyp), _, _) => self.modifier_key = Some(KeyModifier::Hyp),

            (Some(KeyModifier::Alpha), Some(Key::Alpha), _, _) => self.modifier_key = None,
            (_, Some(Key::Alpha), _, _) => self.modifier_key = Some(KeyModifier::Alpha),

            (Some(KeyModifier::Shift), Some(Key::Rcl), _, _) => self.modifier_key = Some(KeyModifier::Sto),

            (Some(KeyModifier::RCL), Some(Key::Rcl), _, _) => self.modifier_key = None,
            (_, Some(Key::Rcl), _, _) => self.modifier_key = Some(KeyModifier::RCL),

            (_, Some(Key::Left) , Some((menu, page)), _) => self.menu.as_mut().unwrap().1 = page.checked_sub(1).unwrap_or(menu.pages()-1),
            (_, Some(Key::Right), Some((menu, page)), _) => self.menu.as_mut().unwrap().1 = (page+1) % menu.pages(),

            // TODO: replace with the length
            (_, Some(Key::Left) , None, _) => self.cursor_at = self.cursor_at.saturating_sub(1),
            (_, Some(Key::Right), None, _) => self.cursor_at = (self.cursor_at+1).min(10),
            (_, Some(Key::Up)   , None, _) => self.cursor_at = 0,
            (_, Some(Key::Down) , None, _) => self.cursor_at = 10,

            (None, Some(key), Some((menu, page)), _) => menu.on_menu_interaction(unsafe { &mut *(&*self as *const Calculator as *mut Calculator) }, *page, key),

            (_, Some(_), _, _) => self.modifier_key = None,
            (_, None, _, _) => (),
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
            "{} {} r∠θR⇔I {:?}",
            self.modifier_key.as_ref().map_or("      ", |a| a.status_name()),
            self.mode.status_name(),
            self.pending_key
            );

        match &self.menu {
            Some((menu, page)) => {
                let (items, fill) = menu.get_page(*page);
                let items_len = items.len();
                let scroll_arrows = menu.pages() > 1;
                top += if scroll_arrows { "←" } else { " " };
                bot += "  ";

                for i in items {
                    top += &" ".repeat(i.2);
                    top += i.0;
                    bot += &i.1.to_string();
                    bot += &" ".repeat(16_usize.div_floor(items_len));
                }

                top += &" ".repeat(fill);
                bot += &" ".repeat(fill);
                top += if scroll_arrows { "→" } else { " " };
            },
            None => {
                top += " απσᴇ𝑒           ";
                bot += "               5.";
                cursor = Some((self.cursor_at, self.insert_mode));
            },
        }

        (stat, top, bot, cursor)
    }
}

impl Mode {
    pub fn status_name(&self) -> &'static str {
        use self::Base::*;
        use Mode::*;
        match self {
            Computation       => "      ",
            Complex           => "CMPLX ",
            Base(Binary)      => "     b",
            Base(Octal)       => "     o",
            Base(Decimal)     => "     d",
            Base(Hexadecimal) => "     h",
            SingleStat        => "SD    ",
            PairedStat        => "REG   ",
            Program           => "PROG  ",
        }
    }
}

impl KeyModifier {
    pub fn status_name(&self) -> &'static str {
        use KeyModifier::*;
        match self {
            //          "SAhMSR"
            Shift    => "S     ",
            Alpha    => " A    ",
            RCL      => "     R",
            Sto      => "    S ",
            ShiftHyp => "S h   ",
            Hyp      => "  h   ",
        }
    }
}

impl Menu {
    pub fn pages(&self) -> usize {
        use Menu::*;
        match self {
            ModeSelect => 2,
        }
    }

    pub fn get_page(&self, page: usize) -> (Vec<MenuItem>, usize) {
        use Menu::*;
        match (self, page) {
            (ModeSelect, 0) => (vec![("COMP", 1, 0), ("CMPLX", 2, 1), ("BASE", 3, 1)], 1),
            (ModeSelect, 1) => (vec![("SD", 4, 1), ("REG", 5, 2), ("PRGM", 6, 2)], 2),

            _ => unreachable!()
        }
    }

    pub fn on_menu_interaction(&self, calc: &mut Calculator, page: usize, key: &Key) {
        match (self, page, key) {
            (Menu::ModeSelect, _, Key::_1) => calc.mode = Mode::Computation,
            (Menu::ModeSelect, _, Key::_2) => calc.mode = Mode::Complex,
            (Menu::ModeSelect, _, Key::_3) => calc.mode = Mode::Base(Base::Decimal),
            (Menu::ModeSelect, _, Key::_4) => calc.mode = Mode::SingleStat,
            (Menu::ModeSelect, _, Key::_5) => calc.mode = Mode::PairedStat,
            (Menu::ModeSelect, _, Key::_6) => calc.mode = Mode::Program,
            _ => return,
        }

        calc.menu = None;
    }
}

type MenuItem = (&'static str, usize, usize);
