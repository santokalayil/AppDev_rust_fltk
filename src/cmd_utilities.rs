use std::process::Command;

pub fn run_command(s: String) {
    let mut cmd = Command::new(s);
    cmd.arg("--version");
    match cmd.output() {
        Ok(o) => {
            println!("THE OUTPUT \n{}", String::from_utf8(o.stdout).unwrap())
        }
        Err(e) => {
            println!("There is an error ie., '{error}'", error = e);
        }
    }
}
