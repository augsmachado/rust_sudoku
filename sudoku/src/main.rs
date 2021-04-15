extern crate piston;

use piston::window::WindowSettings;

fn main() {
    let settings = WindowSettings::new("Sudoku", [512;2])
        .exit_on_esc(true);

    println!("{}", settings.get_exit_on_esc());
}