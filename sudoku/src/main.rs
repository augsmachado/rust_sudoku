extern crate piston;
extern crate glutin_window;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings}; // We use to create an event loop
use glutin_window::GlutinWindow;

fn main() {

    // This function takes two parameters, the title of the window, and the window size.
    let settings = WindowSettings::new("Sudoku", [512;2]).exit_on_esc(true);
    
    // The GlutinWindow struct exposes the underlying API, such that you can write platform specific code when you need it.
    // To create a event loop, you must first make a window mutable.
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    // The event loop is a kind of iterator that polls events from the window and does its own internal logic.
    // This setting tells the event loop to not bother updating at all, and render only when user input is received.
    let mut events = Events::new(EventSettings::new().lazy(true));

    // By default, the event settings are set to a typical FPS shooter. This means that the event loop will consume a lot of
    // energy that is not needed by every kind of game. The default frame rate and update rate settings are specified
    // by DEFAULT_MAX_FPS and DEFAULT_UPS.
    while Some(e) = events.next(&mut window) {

    }
    println!("{}", settings.get_exit_on_esc());
}