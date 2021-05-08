
use fltk::{
    button::*,
    frame::Frame,
    frame::FrameType,
    image::*,
};

// getting asset folder contents
#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

// button

// pub fn gen_button(frame:Frame) -> Button {
//     let mut but = Button::default()
//     .with_size(100, 40)
//     // .size_of(&frame)
//     .center_of(&frame)
    
//     // .Align(Align::Center)
//     .with_label("AppVersion");
//     //.with_pos(10, 10)
//     //.with_size(80, 40)
//     but.set_color(Color::from_rgb(88, 214, 141));
//     but.set_frame(FrameType::RoundUpBox);
    
    
//     // return button
//     but
// }

pub fn gen_login_button() -> Button {
    let mut but = Button::default()
    .with_size(100, 40)
    .with_label("Login");
    but.set_color(Color::from_rgb(88, 214, 141));
    but.set_frame(FrameType::RoundUpBox);
    but
}



pub fn gen_imgbanner(x:i32, y:i32, width: i32, height: i32) -> Frame {
    let mut banner_frame = Frame::new(x, y, width, height, "");
    {
        banner_frame.set_frame(FrameType::FlatBox);
        banner_frame.set_color(Color::from_rgb(0, 120, 254));
        // let banner_img = Assets::get("banner.svg").unwrap();
        // let mut banner_img = SvgImage::from_data(&banner_img).unwrap();
        let mut banner_img = SvgImage::load(&std::path::Path::new("assets\\banner.svg")).unwrap();
        banner_img.scale(width, height, false, true);
        banner_frame.set_image(Some(banner_img));
    }
    banner_frame
}

pub fn gen_output_label_frame(x:i32, y:i32, width: i32, height: i32) -> Frame {
    let mut output_label_frame = Frame::new(x, y, width, height, "Ecclesiastica Login");
    {
        output_label_frame.set_frame(FrameType::FlatBox);
        output_label_frame.set_color(Color::from_rgb(0, 120, 254));
        output_label_frame.set_label_color(Color::from_rgb(200, 200, 200));
    }   
    
    output_label_frame
}