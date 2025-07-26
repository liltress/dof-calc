use crate::lib::core::Lens;
use std::fmt::{self};
use textwrap;

const LEFT_BORDER: &str = "| ";
const RIGHT_BORDER: &str = " |";
const HORZ_BORDER: &str = "-";

#[derive(Debug)]
pub enum MenuItem<'a> {
    Blank,
    Bar,
    Paragraph(&'a str),
    SpecList(&'a Lens),
    Scale(&'a Lens),
}

fn format_paragraph(text: &str, text_width: usize) -> String {
    let mut textout = String::new();
    for line in textwrap::wrap(text, text_width) {
        textout.push('\n');
        textout.push_str(LEFT_BORDER);
        textout.push_str(&line);
        textout.push_str(&" ".repeat(text_width - line.len()));
        textout.push_str(RIGHT_BORDER);
    }
    textout
}

fn display_specs(lens: &Lens, text_width: usize) -> String {
    //! TODO refactor this
    format!(
        "\n{}{: <w$}{}",
        LEFT_BORDER,
        "Lens Name: ".to_string() + &lens.name,
        RIGHT_BORDER,
        w = text_width - 1
    ) + &format!(
        "\n{}{: <w$}{}",
        LEFT_BORDER,
        "Focal Length: ".to_string() + &lens.focal_length.to_string() + "mm",
        RIGHT_BORDER,
        w = text_width - 1
    ) + &format!(
        "\n{}{: <w$}{}",
        LEFT_BORDER,
        "FStop: f".to_string() + &lens.fstop.to_string(),
        RIGHT_BORDER,
        w = text_width - 1
    ) + &format!(
        "\n{}{: <w$}{}",
        LEFT_BORDER,
        "Focused to: ".to_string() + &(lens.focus_distance / 1000.0).to_string() + "m",
        RIGHT_BORDER,
        w = text_width - 1
    )
}

fn display_scale(lens: &Lens, text_width: usize) -> String {
    let hyperfocal = lens.get_hyperfocal_distance();
    let (near, far) = lens.get_field_of_focus();

    let mut scale = String::new();
    scale += "|";
    scale += &(0..text_width - 3).map(|_| "-").collect::<String>();
    scale += "|";

    let mut labels = String::new();
    labels += "0m";
    labels += &((0..text_width
        - ((hyperfocal / 1000.).to_string() + "m").len()
        - RIGHT_BORDER.len()
        - 1)
        .map(|_| " ")
        .collect::<String>());
    labels += &((hyperfocal / 1000.).to_string() + "m");

    format!("\n{}{}{}", LEFT_BORDER, scale, RIGHT_BORDER)
        + &format!("\n{}{}{}", LEFT_BORDER, labels, RIGHT_BORDER)
}

impl fmt::Display for MenuItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let true_width = termion::terminal_size().unwrap().0 as usize;
        let width: usize = true_width - LEFT_BORDER.len() - RIGHT_BORDER.len();
        let outstr: &str = match self {
            Self::Blank => &format!(
                "\n{LEFT_BORDER: <w$}{RIGHT_BORDER}",
                w = width + LEFT_BORDER.len() - 1
            ),
            Self::Bar => &format!("\n{}", HORZ_BORDER.repeat(true_width))[..(true_width)],
            Self::Paragraph(text) => &format_paragraph(text, width - 1),
            Self::SpecList(lens) => &display_specs(lens, width),
            Self::Scale(lens) => &display_scale(lens, width),
        };

        write!(f, "{}", outstr)
    }
}
