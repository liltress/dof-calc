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
        "\n{}Lens Name: {: <w$}{}",
        LEFT_BORDER,
        lens.name,
        RIGHT_BORDER,
        w = text_width - 12
    ) + &format!(
        "\n{}Focal Length: {}mm{: >w$}",
        LEFT_BORDER,
        lens.focal_length * 1000.,
        RIGHT_BORDER,
        w = text_width - (lens.focal_length * 1000.).to_string().len() - 15
    ) + &format!(
        "\n{}FStop: {: <w$}{}",
        LEFT_BORDER,
        lens.fstop,
        RIGHT_BORDER,
        w = text_width - 8
    ) + &format!(
        "\n{}Focused to: {:.1}m{: >w$}",
        LEFT_BORDER,
        lens.focus_distance,
        RIGHT_BORDER,
        w = text_width - 3 - 12
    )
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
            Self::Scale(lens) => "",
        };

        write!(f, "{}", outstr)
    }
}
