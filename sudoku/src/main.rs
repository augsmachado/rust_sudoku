#![deny(missing_docs)]

//! A Sudoku game.

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
/// We use to create an event loop.
use piston::input::RenderEvent;
/// We will handle the render events emitted by the event loop.
use piston::window::WindowSettings;


pub use crate::gameboard::GameBoard;
pub use crate::gameboard_controller::GameBoardController;

mod gameboard;
mod gameboard_controller;

fn main() {
    // Set the window backend wich OpenGL version to use.
    let opengl = OpenGL::V3_2;

    // This function takes two parameters, the title of the window, and the window size.
    let settings = WindowSettings::new("Sudoku", [512; 2])
        .graphics_api(opengl)
        .exit_on_esc(true);
    
        // The GlutinWindow struct exposes the underlying API, such that you can write platform specific code when you need it.
    // To create a event loop, you must first make a window mutable.
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    // The event loop is a kind of iterator that polls events from the window and does its own internal logic.
    // This setting tells the event loop to not bother updating at all, and render only when user input is received.
    let mut events = Events::new(EventSettings::new().lazy(true));

    // The gl object stores shaders and buffers that the OpenGL backend for Piston-Graphics needs to talk with the  GPU.
    let mut gl = GlGraphics::new(opengl);

    // By default, the event settings are set to a typical FPS shooter. This means that the event loop will consume a lot of
    // energy that is not needed by every kind of game. The default frame rate and update rate settings are specified
    // by DEFAULT_MAX_FPS and DEFAULT_UPS.
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            
            // In generic code, the GenericEvent is used because it is easier to reason about the code when there is only one
            // trait constraint. It makes it easier to avoid a lot of work to fix breaking changes, e.g. in nested function calls.
            // The GenericEvent trait also makes it possible to implement custom events. This is important on platforms that
            // require special hardware.
            gl.draw(args.viewport(), |c, g| {
                
                // Clear the window in a white color
                use graphics::clear;

                clear([1.0; 4], g);
            });
        }
    }
    println!("{}", settings.get_exit_on_esc());
}
