#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod document;
mod row;

pub use row::Row;
pub use document::Document;
use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;

fn main() {
    Editor::default().run();
}