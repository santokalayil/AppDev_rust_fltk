use fltk::{
    // app::sleep,
    button::*,
    enums::Color,
    frame::*,
    prelude::*,
}; //


pub enum LoginResult {
    CorrectCredentials,
    InvalidUserName,
    InvalidPassword,
}

pub fn login_valid(username: &str, password: &str) -> LoginResult {
    if username == "santokalayil" {
        if password == "hi" {
            LoginResult::CorrectCredentials
        } else {
            LoginResult::InvalidPassword
        }
    } else {
        LoginResult::InvalidUserName
    }
}

pub fn check_login(
    clone_space_after_loginbtn: &mut Frame, 
    clone_close_button: &mut Button, 
    clone_pack: &mut fltk::group::Pack,
    clone_login_successful_frame: &mut Frame,
    clone_username: &fltk::input::Input,
    clone_password: &fltk::input::SecretInput,
) {
    // use logic::*;
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
}