use terminal_color::{colored_println, Color, TermColor};

fn main() {
    let number = 15;
    colored_println!(
        format!("%A colored number: {}%", number).as_str(),
        TermColor::new(Color::Red, Color::Blue)
    )
}
