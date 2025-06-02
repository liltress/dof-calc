use crate::lib::core;

const RIGHT_BORDER: &str = "  |";
const LEFT_BORDER: &str = "|  ";
const HORZ_BORDER: &str = "_";

enum MenuEntry {
    DofScale(core::Lens),
    LensFormat(core::Lens),
    LensSpec(String, fn(core::Lens) -> f32),
    Submenu(String, Box<Menu>),
    Break,
    Border,
}

struct Menu {
    entries: Vec<MenuEntry>,
    width: usize,
}
impl Menu {
    fn printout(&self) {
        for me in self.entries.iter() {}
    }
}

fn boxify(message: &str, width: usize) -> String {
    let mut out = String::new();
    out.push_str(LEFT_BORDER);
    out.push_str(message);
    out.push_str(
        &std::iter::repeat(' ')
            .take(width - LEFT_BORDER.len() - RIGHT_BORDER.len() - message.len())
            .collect::<String>(),
    );
    out.push_str(RIGHT_BORDER);
    out
}
