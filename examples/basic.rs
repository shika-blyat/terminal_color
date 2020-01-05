use terminal_color::{colored_println, Color, TermColor};

fn main() {
    colored_println!(
        "%This text is red on blue% this is normal %and this one is blue on red%",
        TermColor::new(Color::Red, Color::Blue),
        TermColor::new(Color::Blue, Color::Red)
    )
}
