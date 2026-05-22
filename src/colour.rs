pub enum Colour {
    Red,
    Green,
    Yellow,
    Blue,
}

pub fn paint(colour: Colour, text: &str) -> String {
    let code = match colour {
        Colour::Red => "0;31",
        Colour::Green => "0;32",
        Colour::Yellow => "1;33",
        Colour::Blue => "0;34",
    };
    format!("\x1b[{code}m{text}\x1b[0m")
}
