#![windows_subsystem = "windows"] // comment this line to show console window

// mod cmd_utilities;
mod gui;
// mod python;

// use these if need to embed image
#[macro_use]
extern crate rust_embed;

fn main() {
    // cmd_utilities::run_command("python".to_string()); // argument needs to be added. find some way like vec or list
    // python::show();
    // python::print_from_python_module();
    gui::start_app()
}
