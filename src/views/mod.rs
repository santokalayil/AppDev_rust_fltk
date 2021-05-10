use fltk::{
    app,
    // // app::sleep,
    // button::*,
    // enums::{Align, Color, FrameType},
    // frame::*,
    // image::*,
    // prelude::*,
    // window::*,
}; //


mod elements; // accessing elements folder
mod logic;
pub mod login;

// getting asset folder contents
#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

pub fn start_app() {
    let app = app::App::default(); //.with_scheme(AppScheme::Gtk)
    app::set_visible_focus(false);

    login::login_window();

    // theming
    app::background(250, 250, 255); // this is the app module imported and not he app

    app.run().unwrap();
}
