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