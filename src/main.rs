mod editor;
use editor::Editor;
mod buffer;
mod terminal;
mod view;
fn main() {
    let mut editor = Editor::default();
    Editor::run(&mut editor);
}
