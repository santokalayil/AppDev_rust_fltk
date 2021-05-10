use fltk::{
    app,
    // app::sleep,
    button::*,
    enums::{Align, Color, FrameType},
    frame::*,
    image::*,
    prelude::*,
    window::*,
}; //

mod elements; // accessing elements folder
mod logic;

// getting asset folder contents
#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

pub fn start_app() {
    let window_x: i32 = 100;
    let window_y: i32 = 100;
    let window_width: i32 = 800;
    let window_height: i32 = 600;
    let window_title: &str = "Ecclesiastica v.01";
    let app = app::App::default(); //.with_scheme(AppScheme::Gtk)
    app::set_visible_focus(false);
    let mut win = Window::new(window_x, window_y, window_width, window_height, None)
        .with_label(&window_title);

    let img = Assets::get("app_icon.png").unwrap(); // these 2 lines to embed if not use line after next line only
    let image = PngImage::from_data(&img).unwrap();

    win.set_icon(Some(image));

    let mut frame = Frame::default().size_of(&win);
    frame.set_frame(FrameType::FlatBox);

    let top_banner_frame = elements::gen_output_label_frame(0, 0, frame.width(), 50);
    let mut close_button = Button::default()
        .with_size(20, 20)
        .right_of(&top_banner_frame, -20)
        .with_align(Align::Inside | Align::Center)
        // .with_align(Align::Right)
        .with_label("X");
    close_button.set_color(Color::from_rgb(200, 200, 200));
    close_button.set_frame(FrameType::RFlatBox);
    close_button.set_selection_color(Color::from_rgb(250, 250, 250));
    close_button.hide();

    // login_succesfull message frame that show
    let mut login_successful_frame = fltk::frame::Frame::default()
    .with_label("Login Successful").center_of(&frame);
    login_successful_frame.set_label_color(Color::from_u32(0x0000ff));
    login_successful_frame.set_label_size(28);
    login_successful_frame.hide(); // hidden until successful login

    // login box frame section that contains username input, password input and login button
    let mut pack = fltk::group::Pack::default()
        .center_of(&frame)
        .with_pos(
            frame.x() + frame.width() / 4,
            frame.y() + top_banner_frame.height() + frame.height() / 6,
        )
        .with_size(frame.width() / 2, 400);

    let mut username_label = Frame::new(0, 0, pack.width(), 30, "Username");
    username_label.set_frame(FrameType::FlatBox);

    let mut username = fltk::input::Input::new(0, 0, pack.width(), 30, "");
    username.set_value(&format!("{}", ""));
    username.set_color(Color::from_u32(0xffffff));
    username.set_tooltip("Type your username here");

    let mut password_label = Frame::new(0, 0, pack.width(), 30, "Password");
    password_label.set_frame(FrameType::FlatBox);
    let mut password = fltk::input::SecretInput::new(0, 0, pack.width(), 30, "");
    password.set_value(&format!("{}", ""));
    password.set_color(Color::from_u32(0xffffff));
    password.set_tooltip("Type your password here");

    let mut space_after_password = Frame::new(0, 0, pack.width(), 10, "");
    space_after_password.set_frame(FrameType::FlatBox);
    let mut login_button = elements::gen_login_button();
    let mut space_after_loginbtn = Frame::new(0, 0, pack.width(), 30, "");
    space_after_loginbtn.set_frame(FrameType::FlatBox);
    space_after_loginbtn.set_label_size(16);
    pack.end();

    let mut clone_close_button = close_button.clone();
    let mut clone_pack = pack.clone();
    let mut clone_login_successful_frame = login_successful_frame.clone();
    let mut clone_space_after_loginbtn = space_after_loginbtn.clone();
    let clone_username = username.clone();
    let clone_password = password.clone();

    // this will resized to match the windows size on successful login
    login_button.set_callback(move |_| {
        use logic::*;
        match login_valid(&clone_username.value(), &clone_password.value()) {
            LoginResult::CorrectCredentials => {
                clone_space_after_loginbtn.set_label_color(Color::from_u32(0x00ff00));
                clone_space_after_loginbtn.set_label("Successful!");
                clone_pack.hide();
                clone_close_button.show();
                clone_login_successful_frame.show();  
            },
            LoginResult::InvalidPassword => {
                clone_space_after_loginbtn.set_label_color(Color::from_u32(0xff0000));
                clone_space_after_loginbtn.set_label("Password incorrect");
            },
            LoginResult::InvalidUserName => {
                clone_space_after_loginbtn.set_label_color(Color::from_u32(0xff0000));
                clone_space_after_loginbtn.set_label("Invalid! Try again");
            },

        }
    });

    let mut clone_close_button = close_button.clone();
    close_button.set_callback(move |_| {
        clone_close_button.hide();
        login_successful_frame.hide();
        space_after_loginbtn.set_label("");
        password.set_value("");
        username.set_value("");
        // username.deactivate();
        
        pack.show();
    });


    win.make_resizable(true);
    win.end();
    win.show_with_args(&["-scheme", "+gtk", "-nokbd"]);
    // win.show();

    // theming
    app::background(229, 229, 255); // this is the app module imported and not he app

    app.run().unwrap();
}
