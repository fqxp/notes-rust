use notes::ui::main_window::AppModel;
use relm4::RelmApp;

fn main() -> Result<(), ()> {
    let app = RelmApp::new("de.fqxp.notes");
    app.run::<AppModel>(());

    Ok(())
}
