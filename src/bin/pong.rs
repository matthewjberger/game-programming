extern crate core;
extern crate gl;
extern crate sdl2;

use core::app::{Application2D, Canvas, Color, Window};

fn main() {
    Application2D::run(handle_events, update, render);
}

fn handle_events(_event: sdl2::event::Event) {}

fn update() {}

fn render(canvas: &mut Canvas<Window>) {
    // Change draw color to white
    canvas.set_draw_color(Color::RGB(255, 255, 255));
}
