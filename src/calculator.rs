use crossterm::style::*;

#[derive(Debug)]
pub struct Calculator {
    mode: Mode,
    menu: Option<(Menu, usize)>,

    pending_key: Option<Key>,
    modifier_key: Option<KeyModifier>,

    cursor_at: usize,
    replace_mode: bool,

    user_input: Vec<Token>,
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

type MenuItem = (&'static str, usize, usize);

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
    Shift, ShiftHyp, Alpha, Rcl, Sto, Hyp
}

pub struct DisplayBlock {
    text: String,
    bold: bool, italic: bool
}

macro_rules! display_block {
    ($text: expr) => {
        DisplayBlock {
            text: $text.to_string(),
            bold: false, italic: false
        }
    };
    (b $text: expr) => {
        DisplayBlock {
            text: $text.to_string(),
            bold: true, italic: false
        }
    };
    (i $text: expr) => {
        DisplayBlock {
            text: $text.to_string(),
            bold: false, italic: true
        }
    };

    (b i $text: expr) => {
        DisplayBlock {
            text: $text.to_string(),
            bold: true, italic: true
        }
    };
}

type DisplayBlocks = Vec<DisplayBlock>;

impl Calculator {
    pub fn new() -> Self {
        Self {
            mode: Mode::Computation,
            menu: None,

            pending_key: None,
            modifier_key: None,

            cursor_at: 0,
            replace_mode: false,

            user_input: Vec::with_capacity(99),
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
            KeyCode::Char('-') => Key::Subtract,
            KeyCode::Char('=') => Key::Alpha,
            KeyCode::Enter     => Key::Exe,
            KeyCode::Backspace => Key::Del,

            KeyCode::Char('q') => Key::Prog,
            KeyCode::Char('w') => Key::Fmla,
            KeyCode::Char('e') => Key::Exp,
            KeyCode::Char('t') => Key::PowNegOne,
            KeyCode::Char('y') => Key::Cubed,
            KeyCode::Char('u') => Key::Rcl,
            KeyCode::Char('i') => Key::Eng,
            KeyCode::Char('o') => Key::BracketStart,
            KeyCode::Char('p') => Key::BracketEnd,
            KeyCode::Char(']') => Key::MPlus,
            KeyCode::Char('\\') => Key::Mode,

            KeyCode::Char('a') => Key::Fraction,
            KeyCode::Char('s') => Key::SquareRoot,
            KeyCode::Char('d') => Key::Squared,
            KeyCode::Char('f') => Key::Power,
            KeyCode::Char('g') => Key::Log,
            KeyCode::Char('h') => Key::Ln,
            KeyCode::Char('k') => Key::Ac,
            KeyCode::Char('l') => Key::Add,
            KeyCode::Char(';') => Key::Multiply,

            KeyCode::Char('z') => Key::Negative,
            KeyCode::Char('x') => Key::Base60,
            KeyCode::Char('c') => Key::Hyp,
            KeyCode::Char('v') => Key::Sin,
            KeyCode::Char('b') => Key::Cos,
            KeyCode::Char('n') => Key::Tan,
            KeyCode::Char('m') => Key::Ans,
            KeyCode::Char(',') => Key::Comma,
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

            (Some(KeyModifier::Rcl), Some(Key::Rcl), _, _) => self.modifier_key = None,
            (_, Some(Key::Rcl), _, _) => self.modifier_key = Some(KeyModifier::Rcl),

            (_, Some(Key::Left) , Some((menu, page)), _) => self.menu.as_mut().unwrap().1 = page.checked_sub(1).unwrap_or(menu.pages()-1),
            (_, Some(Key::Right), Some((menu, page)), _) => self.menu.as_mut().unwrap().1 = (page+1) % menu.pages(),

            (_, Some(Key::Left) , None, _) => self.cursor_at = self.cursor_at.saturating_sub(1),
            (_, Some(Key::Right), None, _) => self.cursor_at = (self.cursor_at+1).min(self.user_input.len()),
            (_, Some(Key::Up)   , None, _) => self.cursor_at = 0,
            (_, Some(Key::Down) , None, _) => self.cursor_at = self.user_input.len(),

            (None, Some(_), Some(_), _) => self.on_menu_interaction(),

            (None, Some(Key::Power), None, _) => self.insert(Token::Power),
            (None, Some(Key::SquareRoot), None, _) => self.insert(Token::SquareRoot),
            (Some(KeyModifier::Shift | KeyModifier::ShiftHyp), Some(Key::Ln), None, _) => self.insert(Token::EPower),
            (Some(KeyModifier::Alpha), Some(Key::Ln), None, _) => self.insert(Token::E),
            (None, Some(Key::Fraction), None, _) => self.insert(Token::Fraction),
            (None, Some(Key::_0), None, _) => self.insert(Token::_0),
            (None, Some(Key::_1), None, _) => self.insert(Token::_1),
            (None, Some(Key::_2), None, _) => self.insert(Token::_2),
            (None, Some(Key::_3), None, _) => self.insert(Token::_3),
            (None, Some(Key::_4), None, _) => self.insert(Token::_4),
            (None, Some(Key::_5), None, _) => self.insert(Token::_5),
            (None, Some(Key::_6), None, _) => self.insert(Token::_6),
            (None, Some(Key::_7), None, _) => self.insert(Token::_7),
            (None, Some(Key::_8), None, _) => self.insert(Token::_8),
            (None, Some(Key::_9), None, _) => self.insert(Token::_9),
            (None, Some(Key::Add), None, _) => self.insert(Token::Add),
            (None, Some(Key::Subtract), None, _) => self.insert(Token::Subtract),
            (None, Some(Key::Multiply), None, _) => self.insert(Token::Multiply),
            (None, Some(Key::Divide), None, _) => self.insert(Token::Divide),
            (None, Some(Key::Negative), None, _) => self.insert(Token::Negative),
            (Some(KeyModifier::Shift | KeyModifier::ShiftHyp), Some(Key::Log), None, _) => self.insert(Token::_10Power),

            (None, Some(Key::Del), None, _) => {
                self.cursor_at = self.cursor_at.saturating_sub(1);

                if self.user_input.len() > 0 {
                    self.user_input.remove(self.cursor_at);
                }
            },

            (_, Some(_), _, _) => self.modifier_key = None,
            (_, None, _, _) => (),
        }
    }

    fn insert(&mut self, t: Token) {
        if !self.replace_mode {
            self.user_input.insert(self.cursor_at, t);
            self.cursor_at += 1;
        } else {
            self.user_input[self.cursor_at] = t;
        }
        self.modifier_key = None;
    }

    pub fn get_display(&self) -> (String, DisplayBlocks, String, Option<(usize, bool)>) {
        let mut stat = String::new();
        let mut top  = DisplayBlocks::new();
        let mut bot  = String::new();
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
                top.push(DisplayBlock {
                    text: if scroll_arrows { "←" } else { " " }.to_string(),
                    bold: false, italic: false
                });
                bot += "  ";

                for i in items {
                    top.push(DisplayBlock { text: " ".repeat(i.2), bold: false, italic: false });
                    top.push(DisplayBlock { text: i.0.to_string(), bold: false, italic: false });
                    bot += &i.1.to_string();
                    bot += &" ".repeat(16_usize.div_floor(items_len));
                }

                top.push(DisplayBlock { text: " ".repeat(fill), bold: false, italic: false });
                bot += &" ".repeat(fill);
                top.push(DisplayBlock {
                    text: if scroll_arrows { "→" } else { " " }.to_string(),
                    bold: false, italic: false
                });
            },
            None => {
                top.push(DisplayBlock {
                    text: " ".to_string(),
                    bold: false, italic: false
                });

                let mut cursor_acc = 0;
                let mut cursor_position = 0;
                for (i, el) in self.user_input.iter().enumerate() {
                    let mut blocks = el.as_display_block();
                    cursor_acc = blocks.iter().fold(cursor_acc, |acc, i| i.text.chars().count() + acc);
                    top.append(&mut blocks);

                    if self.cursor_at-1 == i {
                        cursor_position = cursor_acc
                    }
                }
                bot += "            todo!";
                cursor = Some((cursor_position, self.replace_mode));
            },
        }

        (stat, top, bot, cursor)
    }

    pub fn on_menu_interaction(&mut self) {
        macro_rules! map_menu {
            ($($menu: ident page $page: pat, key $($key:ident)|+ => $block: expr),* $(,)?) => {
                match (&self.menu.as_ref().unwrap(), self.pending_key.as_ref().unwrap()) {
                    $(
                        ((Menu::$menu, $page), $(Key::$key)|+) => { $block },
                    )*
                    (_, Key::Ac) => (),
                    _ => return,
                }
            };
        }

        map_menu!(
            ModeSelect page _, key _1 => {
                self.mode = Mode::Computation;
                self.user_input.clear();
                self.cursor_at = 0;
            },
            ModeSelect page _, key _2 => {
                self.mode = Mode::Complex;
                self.user_input.clear();
                self.cursor_at = 0;
            },
            ModeSelect page _, key _3 => {
                self.mode = Mode::Base(Base::Decimal);
                self.user_input.clear();
                self.cursor_at = 0;
            },
            ModeSelect page _, key _4 => {
                self.mode = Mode::SingleStat;
                self.user_input.clear();
                self.cursor_at = 0;
            },
            ModeSelect page _, key _5 => {
                self.mode = Mode::PairedStat;
                self.user_input.clear();
                self.cursor_at = 0;
            },
            ModeSelect page _, key _6 => {
                self.mode = Mode::Program;
                self.user_input.clear();
                self.cursor_at = 0;
            },
        );

        self.menu = None;
    }
}

impl Mode {
    pub const fn status_name(&self) -> &'static str {
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
    pub const fn status_name(&self) -> &'static str {
        use KeyModifier::*;
        match self {
            //          "SAhMSR"
            Shift    => "S     ",
            Alpha    => " A    ",
            Rcl      => "     R",
            Sto      => "    S ",
            ShiftHyp => "S h   ",
            Hyp      => "  h   ",
        }
    }
}

impl Menu {
    pub const fn pages(&self) -> usize {
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
}

impl DisplayBlock {
    pub fn as_styled(&self) -> StyledContent<String> {
        let mut c = self.text.clone().stylize();
        if self.bold { c = c.bold(); }
        if self.italic { c = c.italic(); }
        c
    }
}

#[derive(Debug)]
enum Token {
    Power, SquareRoot, EPower, E, Fraction,
    _0, _1, _2, _3, _4, _5, _6, _7, _8, _9,
    Add, Subtract, Multiply, Divide, Negative,
    _10Power,
}

impl Token {
    pub fn as_display_block(&self) -> Vec<DisplayBlock> {
        use Token::*;
        use display_block as d;
        match self {
            Power           => vec![d!("^(")],
            SquareRoot      => vec![d!("√(")],
            E               => vec![d!(b i "e")],
            EPower          => vec![d!(b i "e"), d!("^(")],
            Fraction        => vec![d!("⅃")],
            _0              => vec![d!("0")],
            _1              => vec![d!("1")],
            _2              => vec![d!("2")],
            _3              => vec![d!("3")],
            _4              => vec![d!("4")],
            _5              => vec![d!("5")],
            _6              => vec![d!("6")],
            _7              => vec![d!("7")],
            _8              => vec![d!("8")],
            _9              => vec![d!("9")],
            Add             => vec![d!("+")],
            Subtract        => vec![d!("–")],
            Multiply        => vec![d!("×")],
            Divide          => vec![d!("÷")],
            Negative        => vec![d!("╶")],
            _10Power        => vec![d!("₁₀^(")],
        }
    }
}
