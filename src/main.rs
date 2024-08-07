mod app;
mod tui;

use std::io;

use crate::app::App;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app = App::default().run(&mut terminal);
    tui::restore()?;
    app
}
