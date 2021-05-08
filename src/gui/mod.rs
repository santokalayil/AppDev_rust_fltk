use fltk::{
    app, button::*, frame::*, window::*, image::*,
    enums::Color, enums::LabelType, enums::Align,
    prelude::*, menu::{MenuBar, MenuItem}, text::TextBuffer,
};  //

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
    // app::set_visible_focus(false);
    let mut win = Window::new(window_x, window_y, window_width, window_height, &window_title,);
    // win.set_color(Color::DarkMagenta);

    // let mut menu = MenuBar::new(0, 0, 800, 40, "hi");
    // menu.set_color(Color::Red);

    // let mut buf: TextBuffer = TextBuffer::default();
    // buf.set_tab_distance(4);



    // MenuItem::new(choices: &[&str])
    // let file = MenuItem::with_label("File");
    // let about = MenuItem::with_label("About");

    let img = Assets::get("app_icon.png").unwrap(); // these 2 lines to embed if not use line after next line only
    let image = PngImage::from_data(&img).unwrap();
    // let image = PngImage::load(&std::path::Path::new("assets/app_icon.png")).unwrap();

    win.set_icon(Some(image));





    //let mut frame = Frame::new(0, 0, window_x, window_y, "HIIIII");
    let mut frame = Frame::default().size_of(&win);
    frame.set_frame(FrameType::FlatBox);
    // frame.set_color(Color::Blue); // now we are setting background in the frame


    // let mut banner_frame = elements::gen_imgbanner(0, 0, frame.width(), 100);
    let mut output_label_frame = elements::gen_output_label_frame(0, 0, frame.width(), 50);
    // let mut but = elements::gen_button(frame);
    let mut close_button = Button::default()
    // .bottom_of(&output_label_frame)
    .with_size(15,15)
    .with_label("x");
    close_button.set_color(Color::from_rgb(255, 0, 0));

    // close_button.hide();


    // let login_box_width: i32 = window_width;
    // let login_box_height: i32 = window_height;

    // let mut pack = fltk::group::Pack::new(200, 200, 400, 400,"Enter Credentials");
    let mut pack = fltk::group::Pack::default()
    .center_of(&frame)
    .with_pos(frame.x() + frame.width()/4,  frame.y()+output_label_frame.height()+frame.height()/6)
    .with_size(frame.width() / 2, 400);
    // .with_label("Enter credentials");
    // pack.set_frame(FrameType::FlatBox);
    // pack.set_color(Color::Cyan);

    let mut username_label = Frame::new(0, 0, pack.width(), 30, "Username");
    // let mut username_label = Frame::default().set_label("UserName");
    username_label.set_frame(FrameType::FlatBox);
    // username_label.set_align(fltk::Align::Left);
    let mut username = fltk::input::Input::new(0, 0, pack.width() ,30, "");
    username.set_value(&format!("{}", ""));
    username.set_color(Color::Red);
    username.set_tooltip("Type your username here");

    let mut password_label = Frame::new(0, 0, pack.width(), 30, "Password");
    password_label.set_frame(FrameType::FlatBox);
    let mut password = fltk::input::SecretInput::new(0, 0, pack.width() ,30, "");
    password.set_value(&format!("{}", ""));
    password.set_tooltip("Type your password here");
    let mut login_button = elements::gen_login_button();
    pack.end();
    // pack.orientation(Align::vertical);

    // close_button.set_callback(move | x | x.hide());
    let mut clone_close_button = close_button.clone();
    let mut clone_output_label_frame = output_label_frame.clone();
    login_button.set_callback( 
        move ||
        if username.value() == "santokalayil".to_string() {
            // pack.clear();
            // username.set_value(&format!("succesfull : {}", "Yes"));
            if password.value() == "hi".to_string() {
                clone_output_label_frame.set_label("Successful Login");
                pack.hide();
                clone_close_button.show();
            }
            else {
                clone_output_label_frame.set_label("password incorrect");
            }
        }
        else {
            clone_output_label_frame.set_label("Invalid! Try again");
        }
    );

    let mut clone_close_button = close_button.clone();
    close_button.set_callback( move || 
        {
            output_label_frame.hide();
            clone_close_button.hide();
        }
    );
    
    // login_button.set_callback(move || logic::login("santokalayil", "hi"));





    win.make_resizable(true);
    win.end();
    win.show_with_args(&["-scheme","+gtk","-nokbd"]);
    // win.show();

    // theming
    app::background(200, 200, 200); // this is the app module imported and not he app
    

    // but.set_callback(move || output_label_frame.set_label("Ecclesiastica v.0.1"));
    app.run().unwrap();
}

