#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Reset,
    Current,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TermColor {
    fg: Color,
    bg: Color,
}
impl TermColor {
    pub fn new(fg: Color, bg: Color) -> Self {
        Self { bg, fg }
    }
}

#[macro_export]
macro_rules! colored_println {
    ( $fmt_string: expr, $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            $crate::print_colors($fmt_string, temp_vec);
        }
    };
}

pub fn print_colors<'a>(string: &'a str, colors: Vec<TermColor>) {
    let mut colors_iter = colors.iter();
    let mut colors = vec![];
    let mut splitted_string: Vec<String> = vec![String::new()];
    let mut in_colored_string = (String::new(), false);
    let mut last_char = '\0';
    for i in string.chars() {
        if in_colored_string.1 {
            if i == '%' && last_char != '\\' {
                colors.push(TermColor {
                    fg: Color::Reset,
                    bg: Color::Reset,
                });
                colors.push(
                    *colors_iter.next().expect(
                        format!(
                            "Missing color specifier for {}",
                            splitted_string.last().unwrap()
                        )
                        .as_str(),
                    ),
                );
                in_colored_string.1 = false;
                splitted_string.push(in_colored_string.0.clone());
                splitted_string.push(String::new());
            } else {
                in_colored_string.0.push(i);
            }
            continue;
        }
        if i == '%' && last_char != '\\' {
            in_colored_string = (String::new(), true);
            continue;
        }
        splitted_string.last_mut().unwrap().push(i);
        last_char = i;
    }
    if splitted_string.len() - 1 != colors.len() {
        panic!("Too much color codes")
    }
    if splitted_string[0].is_empty() {
        splitted_string.remove(0);
    }
    if splitted_string.last().unwrap().is_empty() {
        splitted_string.pop();
    }
    let mut formated_string = String::new();

    for (k, i) in splitted_string.iter().enumerate() {
        let mut string = termcolor_to_code(colors.get(k + 1));
        string.push_str(i);
        string.push_str("\x1b[39;49m");
        formated_string.push_str(string.as_str());
    }
    println!("{}", formated_string);
}
fn termcolor_to_code<'a>(color: Option<&TermColor>) -> String {
    match color {
        Some(TermColor { fg, bg }) => match (fg, bg) {
            (Color::Reset, _) => return String::from("\x1b[39;49m"),
            (_, Color::Reset) => return String::from("\x1b[39;49m"),
            (Color::Current, Color::Current) => String::new(),
            (_, Color::Current) => return format!("\x1b[{}m", color_to_fgcode(fg)),
            (Color::Current, _) => return format!("\x1b[{}m", color_to_bgcode(bg)),
            _ => format!("\x1b[{};{}m", color_to_bgcode(bg), color_to_fgcode(fg)),
        },
        None => return String::from("\x1b[39;49m"),
    }
}
fn color_to_fgcode<'a>(color: &Color) -> &'a str {
    match color {
        Color::Black => "30",
        Color::Red => "31",
        Color::Green => "32",
        Color::Yellow => "33",
        Color::Blue => "34",
        Color::Magenta => "35",
        Color::Cyan => "36",
        Color::White => "37",
        Color::BrightBlack => "90",
        Color::BrightRed => "91",
        Color::BrightGreen => "92",
        Color::BrightYellow => "93",
        Color::BrightBlue => "94",
        Color::BrightMagenta => "95",
        Color::BrightCyan => "96",
        Color::BrightWhite => "97",
        _ => unreachable!(),
    }
}

fn color_to_bgcode<'a>(color: &Color) -> &'a str {
    match color {
        Color::Black => "40",
        Color::Red => "41",
        Color::Green => "42",
        Color::Yellow => "43",
        Color::Blue => "44",
        Color::Magenta => "45",
        Color::Cyan => "46",
        Color::White => "47",
        Color::BrightBlack => "100",
        Color::BrightRed => "101",
        Color::BrightGreen => "102",
        Color::BrightYellow => "103",
        Color::BrightBlue => "104",
        Color::BrightMagenta => "105",
        Color::BrightCyan => "106",
        Color::BrightWhite => "107",
        _ => unreachable!(),
    }
}
