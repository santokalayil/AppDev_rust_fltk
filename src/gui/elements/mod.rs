
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

pub fn gen_login_button() -> Button {
    let mut but = Button::default()
    .with_size(70, 30)
    .with_label("Login");
    but.set_color(Color::from_u32(0x003F87));
    but.set_label_color(Color::from_u32(0xffffff));
    but.set_frame(FrameType::RFlatBox);
    but.set_selection_color(Color::from_u32(0x3498db));
    but
}



// pub fn gen_imgbanner(x:i32, y:i32, width: i32, height: i32) -> Frame {
//     let mut banner_frame = Frame::new(x, y, width, height, "");
//     {
//         banner_frame.set_frame(FrameType::FlatBox);
//         banner_frame.set_color(Color::from_rgb(0, 120, 254));
//         // let banner_img = Assets::get("banner.svg").unwrap();
//         // let mut banner_img = SvgImage::from_data(&banner_img).unwrap();
//         let mut banner_img = SvgImage::load(&std::path::Path::new("assets\\banner.svg")).unwrap();
//         banner_img.scale(width, height, false, true);
//         banner_frame.set_image(Some(banner_img));
//     }
//     banner_frame
// }

pub fn gen_output_label_frame(x:i32, y:i32, width: i32, height: i32) -> Frame {
    let mut output_label_frame = Frame::new(x, y, width, height, "Ecclesiastica Login");
    {
        output_label_frame.set_frame(FrameType::FlatBox);
        output_label_frame.set_color(Color::from_rgb(0, 120, 254));
        output_label_frame.set_label_color(Color::from_u32(0xffffff));
        output_label_frame.set_label_size(24);
        output_label_frame.set_label_font(fltk::Font::Helvetica);
    }   
    
    output_label_frame
}