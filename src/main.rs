use notes::ui::main_window::App;
use relm4::RelmApp;

fn main() -> Result<(), ()> {
    let app = RelmApp::new("de.fqxp.notes");
    app.run_async::<App>(());

    Ok(())
}
