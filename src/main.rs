mod app;
mod device_controls_view;
mod shortcuts;
mod widget_ext;

use relm4::RelmApp;

use crate::app::App;

fn main() {
    let app = RelmApp::new("alsamixer-gui.wagnrd.de");
    app.run::<App>(());
}
